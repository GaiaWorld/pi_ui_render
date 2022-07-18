_$pi.define("app_b/scene/render/model/render_model", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";

    exports.ERenderModelPath = exports.ERenderModelClassName = exports.RenderModel = void 0;
    class RenderModel {
        constructor(scene, rdIndex) {
            this.rdIndex = rdIndex;
            this.scene = scene;
        }
        rdPosition(x, y, z) {
            throw new Error("Method not implemented.");
        }
        rdRotation(x, y, z) {
            throw new Error("Method not implemented.");
        }
        rdScaling(x, y, z) {
            throw new Error("Method not implemented.");
        }
        rdEnable(flag) {
            if (this.model) {
                this.model.setEnabled(flag);
            }
        }
        rdAnimation(animID) {
            throw new Error("Method not implemented.");
        }
        rdAnimationEnd(call) {
            throw new Error("Method not implemented.");
        }
        rdState(state) {
            throw new Error("Method not implemented.");
        }
        rdReset() {
            throw new Error("Method not implemented.");
        }
        rdDestroy() {
            throw new Error("Method not implemented.");
        }
        getPosition() {
            throw new Error("Method not implemented.");
        }
        rdSize(size, sizeForWidth) {
            throw new Error("Method not implemented.");
        }
        static Get(scene, className, rdIndex) {
            let factory = RenderModel.FactoryMap.get(className);
            if (factory) {
                let temp = factory(scene, rdIndex);
                temp.rdEnable(true);
                return temp;
            }
        }
        destroy() {
            this.model.dispose();
            this.scene = undefined;
        }
    }
    exports.RenderModel = RenderModel;
    RenderModel.ClassName = "ModelObj";
    RenderModel.FactoryMap = new Map();
    var ERenderModelClassName;
    (function (ERenderModelClassName) {
        ERenderModelClassName["main"] = "main";
        ERenderModelClassName["eff_map01_01"] = "eff_map01_01";
    })(ERenderModelClassName = exports.ERenderModelClassName || (exports.ERenderModelClassName = {}));
    var ERenderModelPath;
    (function (ERenderModelPath) {
        ERenderModelPath["main"] = "main/";
        ERenderModelPath["eff_map01_01"] = "eff_map01_01/";
    })(ERenderModelPath = exports.ERenderModelPath || (exports.ERenderModelPath = {}));
});