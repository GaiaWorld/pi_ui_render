_$pi.define("app_b/scene/render/render_shadow_manager", ["require", "exports", "module", "pi_babylon/render3d/babylon"], function (require, exports, module, babylon_1) {
    "use strict";

    exports.RenderShadowManager = void 0;
    class RenderShadowManager {
        constructor(scene) {
            this._list = [];
            RenderShadowManager.GeneratorMap.set(scene, this);
        }
        add(mesh) {
            if (this.generator) {
                this._list = undefined;
                this.generator.addShadowCaster(mesh);
            } else {
                this._list.push(mesh);
            }
        }
        remove(mesh) {
            if (this.generator) {
                this._list = undefined;
                this.generator.removeShadowCaster(mesh);
            } else {
                let index = this._list.indexOf(mesh);
                if (index >= 0) {
                    this._list.splice(index, 1);
                }
            }
        }
        init(light) {
            this.generator = new babylon_1.BABYLON.ShadowGenerator(1024, light);
            this._add();
        }
        static Create(scene) {
            let temp = RenderShadowManager.GeneratorMap.get(scene);
            if (!temp) {
                temp = new RenderShadowManager(scene);
            }
            return temp;
        }
        _add() {
            if (this._list) {
                let count = this._list.length;
                for (let i = 0; i < count; i++) {
                    this.generator.addShadowCaster(this._list[i]);
                }
            }
            this._list.length = 0;
            this._list = undefined;
        }
    }
    exports.RenderShadowManager = RenderShadowManager;
    RenderShadowManager.GeneratorMap = new Map();
});