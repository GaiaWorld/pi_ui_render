_$pi.define("app_b/scene/render/camera/render_camera", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";

    exports.ERenderCameraClassName = exports.RenderCamera = void 0;
    class RenderCamera {
        constructor(scene, rdIndex) {
            this.scene = scene;
            this.rdIndex = rdIndex;
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
            if (this.camera) {
                this.camera.setEnabled(flag);
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
            this.scene.removeCamera(this.camera.name);
            this.camera.dispose();
        }
        getPosition() {
            throw new Error("Method not implemented.");
        }
        rdSize(size, sizeForWidth) {
            throw new Error("Method not implemented.");
        }
        static Get(scene, className, rdIndex) {
            let factory = RenderCamera.FactoryMap.get(className);
            if (factory) {
                let temp = factory(scene, rdIndex);
                temp.rdEnable(true);
                return temp;
            }
        }
    }
    exports.RenderCamera = RenderCamera;
    RenderCamera.FactoryMap = new Map();
    var ERenderCameraClassName;
    (function (ERenderCameraClassName) {
        ERenderCameraClassName["indexCamera"] = "indexCamera";
    })(ERenderCameraClassName = exports.ERenderCameraClassName || (exports.ERenderCameraClassName = {}));
});