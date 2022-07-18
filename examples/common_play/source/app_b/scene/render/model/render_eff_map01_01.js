_$pi.define("app_b/scene/render/model/render_eff_map01_01", ["require", "exports", "module", "pi_babylon/model_obj", "./render_model"], function (require, exports, module, model_obj_1, render_model_1) {
    "use strict";

    exports.RenderEffMap0101 = void 0;
    class RenderEffMap0101 extends render_model_1.RenderModel {
        constructor(scene, rdIndex) {
            super(scene, rdIndex);
            let source = RenderEffMap0101.source.get(scene);
            if (!source) {
                RenderEffMap0101.Init(scene);
                source = RenderEffMap0101.source.get(scene);
            }
            // 唯一实例内容,因此没有clone分支
            this.model = source;
        }
        static _Get(scene, rdIndex) {
            let list = RenderEffMap0101.pool.get(scene);
            if (!list) {
                list = [];
                RenderEffMap0101.pool.set(scene, list);
            }
            let renderModel = list.pop();
            if (!renderModel) {
                renderModel = new RenderEffMap0101(scene, rdIndex);
            }
            return renderModel;
        }
        static Init(scene) {
            let source = RenderEffMap0101.source.get(scene);
            if (!source) {
                source = new model_obj_1.ModelObj(RenderEffMap0101.ClassName, scene, {
                    isEffect: true,
                    fileName: RenderEffMap0101.ClassName,
                    path: RenderEffMap0101.Path,
                    animDefault: true,
                    particleAutoStart: true
                });
                RenderEffMap0101.source.set(scene, source);
            }
            return source.loadPromise;
        }
        static Clear(scene) {
            let list = RenderEffMap0101.pool.get(scene);
            if (list) {
                let count = list.length;
                for (let i = 0; i < count; i++) {
                    list[i].destroy();
                }
                list.length = 0;
            }
            RenderEffMap0101.pool.delete(scene);
        }
        rdDestroy() {
            let list = RenderEffMap0101.pool.get(this.scene);
            list.push(this);
            this.model.setEnabled(false);
        }
        rdReset() {
            this.model.setAnim({
                animName: 'eff_map01_01',
                isLoop: true
            });
        }
    }
    exports.RenderEffMap0101 = RenderEffMap0101;
    RenderEffMap0101.ClassName = render_model_1.ERenderModelClassName.eff_map01_01;
    RenderEffMap0101.Path = render_model_1.ERenderModelPath.eff_map01_01;
    RenderEffMap0101.pool = new Map();
    RenderEffMap0101.source = new Map();
});