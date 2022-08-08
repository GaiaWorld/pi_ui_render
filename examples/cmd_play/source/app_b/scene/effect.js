_$pi.define("app_b/scene/effect", ["require", "exports", "module", "pi_babylon/scene_base", "pi_babylon/render3d/babylon", "pi_babylon/scene", "./scene_data", "pi_babylon/scene_tool", "pi_utils/util/frame_mgr", "./utils"], function (require, exports, module, scene_base_1, babylon_1, scene_1, scene_data_1, scene_tool_1, frame_mgr_1, utils_1) {
    "use strict";

    exports.EffectScene = void 0;
    /**
     * 特效场景
     */
    var EffectScene;
    (function (EffectScene) {
        let sceneStruct;
        let camera;
        let modelMap = new Map();
        let boxType; // 宝箱类型
        let tipsBases; // 宝箱动画数字节点
        let tipsMeshs; // 宝箱动画数字
        let topCall;
        let particle;
        let shadowGenerator;
        let EffectModelId;
        (function (EffectModelId) {
            EffectModelId[EffectModelId["BoxFall"] = 10001] = "BoxFall";
            EffectModelId[EffectModelId["BoxStand"] = 10002] = "BoxStand";
            EffectModelId[EffectModelId["BoxOpen"] = 10003] = "BoxOpen";
        })(EffectModelId = EffectScene.EffectModelId || (EffectScene.EffectModelId = {}));
        function init() {
            ensureScene();
            activeLoop();
            createOrthoCamare();
            pause();
        }
        EffectScene.init = init;
        function create() {
            sceneStruct = scene_1.SceneManager.createScene(scene_data_1.SceneData.effectSceneName);
            if (!sceneStruct.impl.metadata) {
                sceneStruct.impl.metadata = {};
            }
            // remove listenter
            const scene = sceneStruct.impl;
            scene_1.SceneManagerData.canvas.removeEventListener(`pointermove`, scene._inputManager._onPointerMove);
            scene_1.SceneManagerData.canvas.removeEventListener(`pointerdown`, scene._inputManager._onPointerDown);
            window.removeEventListener(`pointerup`, scene._inputManager._onPointerUp);
            sceneStruct.impl.metadata.gltfAsLeftHandedSystem = true;
            sceneStruct.impl.ambientColor = new babylon_1.BABYLON.Color3(0, 0, 0);
            sceneStruct.registerBeforeRenderCall(sceneStruct => {
                // 显式设置为false，默认true
                sceneStruct.impl.autoClear = false;
                sceneStruct.impl.clearColor = new babylon_1.BABYLON.Color4(0, 0, 0, 0);
            });
            const plan = babylon_1.BABYLON.Mesh.CreatePlane('always_stay', 0.001, sceneStruct.impl);
            const mat = new babylon_1.BABYLON.StandardMaterial('always_stay', sceneStruct.impl);
            plan.position.z = -100; //(false);
            plan.material = mat;
            plan.alwaysSelectAsActiveMesh = true;
            mat.disableLighting = false;
            mat.diffuseColor = babylon_1.BABYLON.Color3.Black();
            mat.emissiveColor = babylon_1.BABYLON.Color3.Black();
            loadScene();
        }
        function loadScene() {
            // 避免第一次进入游戏时loadingSceen通知两次加载事件
            babylon_1.BABYLON.SceneLoader.ShowLoadingScreen = false;
        }
        function renderLoop() {
            sceneStruct.render();
        }
        function activeLoop() {
            if (frame_mgr_1.getGlobal().isPermanent(renderLoop)) return;
            frame_mgr_1.getGlobal().setPermanent(renderLoop);
        }
        function ensureScene() {
            if (sceneStruct) return;
            create();
            window.sceneStruct = sceneStruct;
        }
        // 正交相机
        function createOrthoCamare() {
            const cameraName = 'effectOrthoCamera';
            if (sceneStruct.cameraMap.has(cameraName)) return;
            camera = new babylon_1.BABYLON.FreeCamera(cameraName, new babylon_1.BABYLON.Vector3(0, 0, 0), sceneStruct.impl);
            camera.mode = babylon_1.BABYLON.FreeCamera.ORTHOGRAPHIC_CAMERA;
            const SW = scene_1.SceneManagerData.canvas.width,
                  SH = scene_1.SceneManagerData.canvas.height;
            const size = scene_tool_1.CameraTool.getAccurateSize(1.65, SW, SH);
            scene_tool_1.CameraTool.changeCameraOrth(camera, size, SW, SH);
            sceneStruct.addCamera(camera);
            sceneStruct.setCurrCamera(cameraName);
            camera.position = new babylon_1.BABYLON.Vector3(0, 8, 11.5);
            camera.rotation = new babylon_1.BABYLON.Vector3(35 / 180 * Math.PI, Math.PI, 0);
            activeScene();
        }
        function isActive() {
            if (!sceneStruct) return false;
            return sceneStruct.renderFlag === scene_base_1.SceneRenderFlags.active;
        }
        function activeScene() {
            if (isActive()) return;
            if (!sceneStruct) return console.error('EffectScene is disposed!!!');
            sceneStruct === null || sceneStruct === void 0 ? void 0 : sceneStruct.active();
        }
        /**
         * 获取模型位置
         * @param y 模型中心点美术制作在底部，为居中0.6为粗略估计,可调整
         */
        function getModelScreenCoord(modelId, y) {
            const model = modelMap.get(modelId);
            if (!model) throw Error('no model!!!!');
            const position = this.model.rootImpl.getChildMeshes()[0].getAbsolutePosition();
            const coord = utils_1.sceneCoordConvert(position.x, y || 0.6, position.z, sceneStruct);
            return coord;
        }
        EffectScene.getModelScreenCoord = getModelScreenCoord;
        // 宝箱动画名称
        let BoxAnimation;
        (function (BoxAnimation) {
            BoxAnimation["OPEN"] = "eff_ui_baoxiang_dakai";
            BoxAnimation["FALL"] = "eff_ui_baoxiang_diaoluo";
            BoxAnimation["STAND"] = "eff_ui_baoxiang_daiji";
        })(BoxAnimation = EffectScene.BoxAnimation || (EffectScene.BoxAnimation = {}));
        // 获取开箱动画控制器
        function getBoxAnimalControl(_boxType) {
            var _a, _b, _c;
            activeScene();
            boxType = _boxType;
            tipsBases = new Map();
            tipsMeshs = new Map();
            // init load model
            (_a = modelMap.get(EffectModelId.BoxFall)) === null || _a === void 0 ? void 0 : _a.dispose();
            (_b = modelMap.get(EffectModelId.BoxStand)) === null || _b === void 0 ? void 0 : _b.dispose();
            (_c = modelMap.get(EffectModelId.BoxOpen)) === null || _c === void 0 ? void 0 : _c.dispose();
            modelMap.set(EffectModelId.BoxFall, insertBoxMesh(BoxAnimation.FALL));
            modelMap.set(EffectModelId.BoxStand, insertBoxMesh(BoxAnimation.STAND));
            modelMap.set(EffectModelId.BoxOpen, insertBoxMesh(BoxAnimation.OPEN));
            return {
                play: playBoxAnimation,
                setNum: num => {
                    tipsMeshs.forEach(v => v.dispose());
                    tipsMeshs.clear();
                    tipsBases.forEach((v, k) => {
                        const { plane } = utils_1.createTextPlane({ tips: `${num}`, strokeColor: '#f94b62', scene: sceneStruct.impl });
                        plane.position = new babylon_1.BABYLON.Vector3(0, 0, 0);
                        plane.scaling = new babylon_1.BABYLON.Vector3(-0.15, -0.15, -1);
                        plane.parent = v;
                        tipsMeshs.set(k, plane);
                    });
                },
                setTopCall(call) {
                    topCall = call;
                },
                getPosition() {
                    const coord = utils_1.sceneCoordConvert(-0.15, -0.96, 1, sceneStruct);
                    return coord;
                }
            };
        }
        EffectScene.getBoxAnimalControl = getBoxAnimalControl;
        // 播放开箱动画
        function playBoxAnimation(animName, endCall) {
            if (animName === BoxAnimation.FALL) {
                playBoxFall(endCall);
            } else if (animName === BoxAnimation.STAND) {
                playBoxStand(endCall);
            } else if (animName === BoxAnimation.OPEN) {
                playBoxOpen(endCall);
            }
        }
        function playBoxFall(endCall) {
            let model = modelMap.get(EffectModelId.BoxFall);
            let standModel = modelMap.get(EffectModelId.BoxStand);
            let openModel = modelMap.get(EffectModelId.BoxOpen);
            const playFn = () => {
                particle && particle.stop();
                standModel === null || standModel === void 0 ? void 0 : standModel.setEnabled(false);
                openModel === null || openModel === void 0 ? void 0 : openModel.setEnabled(false);
                model.setEnabled(true);
                model.setAnim({
                    animName: BoxAnimation.FALL,
                    isLoop: false,
                    endCall: () => {
                        endCall && endCall();
                    }
                });
            };
            if (!model) {
                return;
            } else {
                playFn();
            }
        }
        function playBoxStand(endCall) {
            let model = modelMap.get(EffectModelId.BoxStand);
            let openModel = modelMap.get(EffectModelId.BoxOpen);
            let fallModel = modelMap.get(EffectModelId.BoxFall);
            const playFn = () => {
                particle && particle.start();
                openModel === null || openModel === void 0 ? void 0 : openModel.setEnabled(false);
                fallModel === null || fallModel === void 0 ? void 0 : fallModel.setEnabled(false);
                model.setEnabled(true);
                model.setAnim({
                    animName: BoxAnimation.STAND,
                    isLoop: true,
                    endCall
                });
            };
            if (!model) {
                return;
            } else {
                playFn();
            }
        }
        function playBoxOpen(endCall) {
            let model = modelMap.get(EffectModelId.BoxOpen);
            let standModel = modelMap.get(EffectModelId.BoxStand);
            let fallModel = modelMap.get(EffectModelId.BoxFall);
            const playFn = () => {
                // 若是播放
                particle && particle.stop();
                standModel === null || standModel === void 0 ? void 0 : standModel.setEnabled(false);
                fallModel === null || fallModel === void 0 ? void 0 : fallModel.setEnabled(false);
                model.setEnabled(true);
                model.setAnim({
                    animName: BoxAnimation.OPEN,
                    isLoop: false,
                    endCall: () => {
                        playBoxStand();
                        endCall && endCall();
                    }
                });
            };
            if (!model) {
                return;
            } else {
                playFn();
            }
        }
        function getBoxTexture(modelName) {
            if (boxType === 3) return;
            const i = modelName === BoxAnimation.FALL ? 0 : 1;
            const data = [['bx_lanseguanbi', 'bx_lansekaiqi'], ['bx_ziseguanbi', 'bx_zisekaiqi']];
            return data[boxType - 1][i];
        }
        function insertBoxMesh(modelName, playFn) {
            const boxImgTextureName = getBoxTexture(modelName);
            return sceneStruct.insertMesh(`boxEffect${scene_data_1.IdManager.getId()}`, {
                path: `eff_ui_baoxiang/`,
                fileName: modelName,
                isEffect: true,
                imageSolts: boxImgTextureName ? [[`texture${scene_data_1.IdManager.getId()}`, 0, `../../images/${boxImgTextureName}.png`, true]] : undefined,
                insertedCall: model => {
                    model.setScale([0.5, 0.55, 1]);
                    let bx2, bx1;
                    model.rootImpl.getChildMeshes().forEach(mesh => {
                        if (mesh.name === 'bx1') bx1 = mesh;
                        if (mesh.name === 'bx2') bx2 = mesh;
                        if (mesh.name === 'paizi') tipsBases.set(modelName, mesh);
                        mesh.material.ambientColor = babylon_1.BABYLON.Color3.White();
                    });
                    if (modelName === BoxAnimation.STAND) {
                        particle = model.particleSysMap.get('xingxing_particle');
                    }
                    // 打开动画的播放到最高时监听
                    if (modelName === BoxAnimation.OPEN) {
                        const animation = bx2.animations.find(anim => anim.targetProperty === 'position');
                        const topFrame = animation.getKeys()[0].frame;
                        var event = new babylon_1.BABYLON.AnimationEvent(topFrame, () => {
                            topCall && topCall();
                        }, true);
                        animation.addEvent(event);
                    }
                    playFn && playFn();
                }
            });
        }
        function pause(modelId) {
            if (modelId) {
                const model = modelMap.get(modelId);
                model === null || model === void 0 ? void 0 : model.setEnabled(false);
            } else {
                modelMap.forEach(model => {
                    model === null || model === void 0 ? void 0 : model.setEnabled(false);
                    sceneStruct === null || sceneStruct === void 0 ? void 0 : sceneStruct.pause();
                });
            }
        }
        EffectScene.pause = pause;
        function active(modelId) {
            if (modelId) {
                const model = modelMap.get(modelId);
                model === null || model === void 0 ? void 0 : model.setEnabled(true);
                sceneStruct === null || sceneStruct === void 0 ? void 0 : sceneStruct.active(); // case: before: all pause, after: one active
            } else {
                modelMap.forEach(model => {
                    model === null || model === void 0 ? void 0 : model.setEnabled(true);
                    sceneStruct === null || sceneStruct === void 0 ? void 0 : sceneStruct.active();
                });
            }
        }
        EffectScene.active = active;
        function dispose(modelId) {
            if (modelId) {
                const model = modelMap.get(modelId);
                model === null || model === void 0 ? void 0 : model.setEnabled(false);
                model === null || model === void 0 ? void 0 : model.dispose();
                modelMap.delete(modelId);
            } else {
                modelMap.forEach(model => model.dispose());
                modelMap.clear();
                scene_1.SceneManagerData.sceneMap.delete(sceneStruct.name);
                sceneStruct === null || sceneStruct === void 0 ? void 0 : sceneStruct.dispose();
                sceneStruct = undefined;
                frame_mgr_1.getGlobal().clearPermanent(renderLoop);
            }
        }
        EffectScene.dispose = dispose;
        function freezeModelObj(modelId) {
            const _model = modelMap.get(modelId);
            modelMap.delete(modelId);
            return _model;
        }
        EffectScene.freezeModelObj = freezeModelObj;
        function unfreezeModelObj(modelId, modelObj) {
            modelMap.set(modelId, modelObj);
        }
        EffectScene.unfreezeModelObj = unfreezeModelObj;
    })(EffectScene = exports.EffectScene || (exports.EffectScene = {}));
});