_$pi.define("app_b/scene/main", ["require", "exports", "module", "pi_babylon/render3d/babylon", "./scene_data", "./ext", "pi_babylon/scene", "./scene", "pi_babylon/babylondownload", "./effect", "pi_common/control_panel/babylon_control_panel/main", "./render/model/render_main", "./render/render_shadow_manager", "./render/model/render_eff_map01_01", "./render/render_scene_manager", "./render/camera/render_camera"], function (require, exports, module, babylon_1, scene_data_1, ext_1, scene_1, scene_2, babylondownload_1, effect_1, main_1, render_main_1, render_shadow_manager_1, render_eff_map01_01_1, render_scene_manager_1, render_camera_1) {
    "use strict";

    exports.IndexScene = void 0;
    /**
     * 主场景
     */
    var IndexScene;
    (function (IndexScene) {
        let sceneStructName = scene_data_1.SceneData.mainSceneName;
        let camera;
        let lastClickRayId;
        let lastClickTimer;
        let bgMesh;
        let loadCount = 0; //防止主场景加载和babylon加载完成异步问题
        // 可被滑动事件响应的模型类型
        const MoveWhiteList = [scene_data_1.MODEL_TYPE.ANIMAL, scene_data_1.MODEL_TYPE.VISITOR, scene_data_1.MODEL_TYPE.NPC];
        // 主场景初始化
        function init() {
            const loading = new babylondownload_1.BABYLONLoading();
            scene_1.SceneManagerData.engine.loadingScreen = loading;
            loading.hideLoadingUI = () => {};
            // 在创建前注册
            render_scene_manager_1.RenderSceneManager.SceneInitCallMap.set(render_scene_manager_1.ERenderSceneName.mainSceneName, function (sceneStruct) {
                if (!sceneStruct.impl.metadata) {
                    sceneStruct.impl.metadata = {};
                }
                sceneStruct.impl.metadata.gltfAsLeftHandedSystem = true;
                sceneStruct.impl.ambientColor = new babylon_1.BABYLON.Color3(0, 0, 0);
                sceneStruct.impl.autoClear = false;
                sceneStruct.impl.autoClearDepthAndStencil = false;
                // 不清屏，清屏统一交给外部管理（外部最清楚应该何时清屏），否则容易出现重复清屏，导致闪屏、花屏等各种问题
                // sceneStruct.registerBeforeRenderCall(sceneStruct => {
                //     sceneStruct.impl.autoClear = true;
                //     sceneStruct.impl.clearColor = new BABYLON.Color4(0, 0, 0, 0);
                // });
                // 注册主场景事件
                handleMove && sceneStruct.addMoveListen(handleMove);
                handleUp && sceneStruct.addUpListen(handleUp);
                handleClick && sceneStruct.addClickListen(handleClick);
                scene_2.ProjectCustomTool.formatEvent(sceneStruct, scene_1.SceneManagerData.canvas);
                window.mainscene = sceneStruct;
                // BABYLON 性能面板初始化
                main_1.BABYLONControlPanel.Init(sceneStruct);
                // 预加载内容
                render_main_1.RenderMain.Init(sceneStruct);
                render_eff_map01_01_1.RenderEffMap0101.Init(sceneStruct);
                // shadow 创建
                render_shadow_manager_1.RenderShadowManager.Create(sceneStruct);
                // 设置为主场景
                scene_1.SceneManager.setMainScene(sceneStruct);
                // 创建相机
                handleCamera();
                // 激活
                sceneStruct.active();
                ensureLoadedEvent();
            });
            render_scene_manager_1.RenderSceneManager.Create(render_scene_manager_1.ERenderSceneName.mainSceneName);
            loadScene();
        }
        IndexScene.init = init;
        // 通知加载完成
        function ensureLoadedEvent() {
            if (loadCount == 1) return scene_data_1.bus.notify(scene_data_1.SceneEvent.SCENE_LOADED, []);
            loadCount += 1;
        }
        // 加载主场景
        function loadScene() {
            const MAIN_MODEL_INDEX = render_scene_manager_1.RenderSceneManager.RenderObj(render_scene_manager_1.ERenderSceneName.mainSceneName, render_main_1.RenderMain.ClassName);
            const WATER_EFFECT_INDEX = render_scene_manager_1.RenderSceneManager.RenderObj(render_scene_manager_1.ERenderSceneName.mainSceneName, render_eff_map01_01_1.RenderEffMap0101.ClassName);
            // sceneStruct.importScene('main', {
            //     path: 'main/',
            //     fileName: `main`,
            //     loadedCall: model => {
            //         model.scene.impl.meshes.forEach(mesh => {
            //             if (mesh.name === 'mesh_bg01') {
            //                 bgMesh = mesh;
            //                 bgMesh.alphaIndex = ALPHA_INDEX.BG;
            //                 (<BABYLON.StandardMaterial>bgMesh.material).diffuseColor = new BABYLON.Color3(0.6588235, 0.6588235, 0.6588235)
            //             }
            //             mesh.receiveShadows = true;
            //         });
            //         let light = <BABYLON.DirectionalLight>sceneStruct.impl.lights.find(light => light.name === 'Directional Light3');
            //         light.shadowMaxZ = 100;
            //         light.shadowMinZ = -10;
            //         const shadowGenerator = new BABYLON.ShadowGenerator(2048, light);
            //         shadowGenerator.usePercentageCloserFiltering = true;
            //         shadowGenerator.filteringQuality = BABYLON.ShadowGenerator.QUALITY_LOW;
            //         SceneData.shadowGenerator = shadowGenerator;
            //         shadowGenerator.setDarkness(0.6);
            //         loaded();
            //     }
            // });
            // sceneStruct.insertMesh('waterEffect', {
            //     isEffect: true,
            //     path: 'eff_map01_01/',
            //     fileName: 'eff_map01_01',
            //     particleAutoStart: true,
            //     insertedCall(model) {
            //         model.setAnim({
            //             animName: 'eff_map01_01',
            //             isLoop: true
            //         });
            //     }
            // });
            loaded();
        }
        // 初始化相机
        function handleCamera() {
            const INDEX_CAMERA = render_scene_manager_1.RenderSceneManager.RenderObj(render_scene_manager_1.ERenderSceneName.mainSceneName, render_camera_1.ERenderCameraClassName.indexCamera);
            let temp = render_scene_manager_1.RenderSceneManager.RenderObjMap.get(INDEX_CAMERA);
            temp.rdPosition(0, 8, 11.5);
            temp.rdRotation(35 / 180 * Math.PI, Math.PI, 0);
            temp.rdSize(scene_data_1.MAP_LIMIT[0]);
            temp = undefined;
            // // 创建光照和相机
            // camera = new BABYLON.FreeCamera('indexCamera', new BABYLON.Vector3(0, 0, 0), sceneStruct.impl);
            // camera.mode = BABYLON.FreeCamera.ORTHOGRAPHIC_CAMERA;
            // const SW = SceneManagerData.canvas.width,
            //     SH = SceneManagerData.canvas.height;
            // const size = CameraTool.getAccurateSize(MAP_LIMIT[0], SW, SH);
            // CameraTool.changeCameraOrth(camera, size, SW, SH);
            // sceneStruct.addCamera(camera);
            // sceneStruct.setCurrCamera('indexCamera');
            // camera.position = new BABYLON.Vector3(0, 8, 11.5);
            // camera.rotation = new BABYLON.Vector3((35 / 180) * Math.PI, Math.PI, 0);
        }
        // 主场景加载完成后置处理建筑，动物，游客的初始化
        function loaded() {
            // sceneStruct.active();
            // SceneManager.setMainScene(sceneStruct);
            ensureLoadedEvent();
            effect_1.EffectScene.init();
        }
        // 场景滑动事件
        function handleMove(info) {
            let { hit, target, pickedMesh } = info.rayInfo;
            target = hit && target && (target.rayID ? target : target.parent) || pickedMesh && pickedMesh.rayID && pickedMesh;
            if (!target) return;
            if (lastClickRayId && lastClickRayId === target.rayID) {
                if (!lastClickTimer) {
                    lastClickTimer = setTimeout(() => {
                        lastClickRayId = undefined;
                        clearTimeout(lastClickTimer);
                        lastClickTimer = null;
                    }, 1000);
                }
                return;
            }
            ;
            lastClickRayId = target.rayID;
            let model = ext_1.ModelManager.getModelById(target.rayID);
            if (MoveWhiteList.indexOf(model.modelType) > -1) {
                model && model.onClick();
            }
        }
        // 场景点击事件
        function handleClick(info) {
            const { hit, target, pickedMesh } = info.rayInfo;
            let rtarget = hit && target && (target.rayID ? target : target.parent) || pickedMesh && pickedMesh.rayID && pickedMesh;
            if (!rtarget) return;
            let model = ext_1.ModelManager.getModelById(rtarget.rayID);
            model && model.onClick(1);
        }
        function handleUp() {
            lastClickRayId = void 0;
        }
        // 模拟点击
        function fakeClick(rayID) {
            let model = ext_1.ModelManager.getModelById(rayID);
            model && model.onClick();
        }
        IndexScene.fakeClick = fakeClick;
        // 主场景暂停
        function pause() {
            ext_1.ModelManager.pause();
            render_scene_manager_1.RenderSceneManager.Pause(render_scene_manager_1.ERenderSceneName.mainSceneName);
        }
        IndexScene.pause = pause;
        // 主场景激活
        function active() {
            render_scene_manager_1.RenderSceneManager.Active(render_scene_manager_1.ERenderSceneName.mainSceneName);
            ext_1.ModelManager.active();
            scene_1.SceneManager.setMainScene(undefined, render_scene_manager_1.ERenderSceneName.mainSceneName);
        }
        IndexScene.active = active;
    })(IndexScene = exports.IndexScene || (exports.IndexScene = {}));
});