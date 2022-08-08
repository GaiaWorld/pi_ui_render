_$pi.define("app_b/scene/render/render_scene_manager", ["require", "exports", "module", "pi_babylon/scene", "./camera/render_camera", "./camera/render_index", "./counter", "./model/render_eff_map01_01", "./model/render_main", "./model/render_model", "./render_shadow_manager"], function (require, exports, module, scene_1, render_camera_1, render_index_1, counter_1, render_eff_map01_01_1, render_main_1, render_model_1, render_shadow_manager_1) {
    "use strict";

    exports.ERenderSceneName = exports.RenderSceneManager = void 0;
    /**
     * 场景渲染管理
     */
    class RenderSceneManager {
        static Create(scenename) {
            const sceneStruct = scene_1.SceneManager.createScene(scenename);
            const initCall = RenderSceneManager.SceneInitCallMap.get(scenename);
            if (initCall) {
                initCall(sceneStruct);
            }
        }
        static Active(scenename) {
            scene_1.SceneManagerData.sceneMap.get(scenename).active();
        }
        static Pause(scenename) {
            scene_1.SceneManagerData.sceneMap.get(scenename).pause();
        }
        /**
         * 添加 RenderModel 类型资源的接口
         * @param scenename
         * @param className
         * @returns
         */
        static RenderObj(scenename, className) {
            const scene = scene_1.SceneManagerData.sceneMap.get(scenename);
            const rdIndex = RenderSceneManager.Counter.get();
            let obj;
            if (render_model_1.ERenderModelClassName[className] != undefined) {
                obj = render_model_1.RenderModel.Get(scene, className, rdIndex);
            } else if (render_camera_1.ERenderCameraClassName[className] != undefined) {
                obj = render_camera_1.RenderCamera.Get(scene, className, rdIndex);
            }
            RenderSceneManager.RenderObjMap.set(rdIndex, obj);
            return rdIndex;
        }
        /**
         * 添加 RenderModel 类型资源的接口
         * @param rdIndex
         */
        static RenderObjDispose(rdIndex) {
            let obj = RenderSceneManager.RenderObjMap.get(rdIndex);
            if (obj) {
                obj.rdDestroy();
            }
            RenderSceneManager.RenderObjMap.delete(rdIndex);
        }
        /**
         * 添加 RenderShadowGenerator 类型资源的接口
         * @param scenename
         */
        static RenderShadowGenerator(scenename) {
            const scene = scene_1.SceneManagerData.sceneMap.get(scenename);
            render_shadow_manager_1.RenderShadowManager.Create(scene);
        }
    }
    exports.RenderSceneManager = RenderSceneManager;
    /**
     * RenderObj 的 ID 生成器
     */
    RenderSceneManager.Counter = new counter_1.Counter();
    RenderSceneManager.RenderObjMap = new Map();
    RenderSceneManager.SceneInitCallMap = new Map();
    /**
     * 场景名称限制
     */
    var ERenderSceneName;
    (function (ERenderSceneName) {
        ERenderSceneName["mainSceneName"] = "MAIN_SCENE";
        ERenderSceneName["animSceneName"] = "ANIM_SCENE";
        ERenderSceneName["syntSceneName"] = "SYNT_SCENE";
        ERenderSceneName["effectSceneName"] = "EFFECT_SCENE";
        ERenderSceneName["lightSceneName"] = "LIGHT_SCENE";
    })(ERenderSceneName = exports.ERenderSceneName || (exports.ERenderSceneName = {}));
    render_model_1.RenderModel.FactoryMap.set(render_main_1.RenderMain.ClassName, render_main_1.RenderMain._Get);
    render_model_1.RenderModel.FactoryMap.set(render_eff_map01_01_1.RenderEffMap0101.ClassName, render_eff_map01_01_1.RenderEffMap0101._Get);
    render_camera_1.RenderCamera.FactoryMap.set(render_index_1.RenderIndexCamera.ClassName, render_index_1.RenderIndexCamera._Get);
});