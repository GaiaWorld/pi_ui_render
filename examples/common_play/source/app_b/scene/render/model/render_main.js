_$pi.define("app_b/scene/render/model/render_main", ["require", "exports", "module", "pi_babylon/model_obj", "pi_babylon/render3d/babylon", "../render_shadow_manager", "./render_model"], function (require, exports, module, model_obj_1, babylon_1, render_shadow_manager_1, render_model_1) {
    "use strict";

    exports.RenderMain = void 0;
    class RenderMain extends render_model_1.RenderModel {
        constructor(scene, rdIndex) {
            super(scene, rdIndex);
            let source = RenderMain.source.get(scene);
            if (!source) {
                RenderMain.Init(scene);
                source = RenderMain.source.get(scene);
            }
            // 唯一实例内容,因此没有clone分支
            this.model = source;
        }
        static _Get(scene, rdIndex) {
            let list = RenderMain.pool.get(scene);
            if (!list) {
                list = [];
                RenderMain.pool.set(scene, list);
            }
            let renderModel = list.pop();
            if (!renderModel) {
                renderModel = new RenderMain(scene, rdIndex);
            }
            return renderModel;
        }
        static Init(scene) {
            let source = RenderMain.source.get(scene);
            if (!source) {
                source = new model_obj_1.ModelObj(RenderMain.ClassName, scene, {
                    fileName: RenderMain.ClassName,
                    path: RenderMain.Path,
                    animDefault: true,
                    particleAutoStart: true,
                    insertedCall: () => {
                        source.meshMap.forEach(mesh => {
                            if (mesh.name === 'mesh_bg01') {
                                mesh.material.diffuseColor = new babylon_1.BABYLON.Color3(0.6588235, 0.6588235, 0.6588235);
                            }
                            mesh.receiveShadows = true;
                        });
                        let light = scene.impl.lights.find(light => light.name === 'Directional Light3');
                        light.shadowMaxZ = 100;
                        light.shadowMinZ = -10;
                        const shadowGenerator = render_shadow_manager_1.RenderShadowManager.GeneratorMap.get(scene);
                        shadowGenerator.init(light);
                    }
                });
                RenderMain.source.set(scene, source);
            }
            return source.loadPromise;
        }
        static Clear(scene) {
            let list = RenderMain.pool.get(scene);
            if (list) {
                let count = list.length;
                for (let i = 0; i < count; i++) {
                    list[i].destroy();
                }
                list.length = 0;
            }
            RenderMain.pool.delete(scene);
        }
        rdDestroy() {
            let list = RenderMain.pool.get(this.scene);
            list.push(this);
            this.rdEnable(false);
        }
    }
    exports.RenderMain = RenderMain;
    RenderMain.ClassName = render_model_1.ERenderModelClassName.main;
    RenderMain.Path = render_model_1.ERenderModelPath.main;
    RenderMain.pool = new Map();
    RenderMain.source = new Map();
});