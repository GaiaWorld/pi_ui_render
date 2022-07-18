_$pi.define("app_b/scene/render/camera/render_index", ["require", "exports", "module", "pi_babylon/render3d/babylon", "pi_babylon/scene", "pi_babylon/scene_tool", "./render_camera"], function (require, exports, module, babylon_1, scene_1, scene_tool_1, render_camera_1) {
    "use strict";

    exports.RenderIndexCamera = void 0;
    class RenderIndexCamera extends render_camera_1.RenderCamera {
        constructor(scene, rdIndex) {
            super(scene, rdIndex);
            // 创建光照和相机
            const camera = new babylon_1.BABYLON.FreeCamera(RenderIndexCamera.ClassName, new babylon_1.BABYLON.Vector3(0, 0, 0), scene.impl);
            camera.mode = babylon_1.BABYLON.FreeCamera.ORTHOGRAPHIC_CAMERA;
            scene.addCamera(camera);
            scene.setCurrCamera(camera.name);
            this.camera = camera;
        }
        static _Get(scene, rdIndex) {
            let temp = RenderIndexCamera.pool.get(scene.name);
            if (!temp) {
                temp = new RenderIndexCamera(scene, rdIndex);
                RenderIndexCamera.pool.set(scene.name, temp);
            }
            return temp;
        }
        rdSize(size, sizeForWidth) {
            const SW = scene_1.SceneManagerData.canvas.width,
                  SH = scene_1.SceneManagerData.canvas.height;
            size = scene_tool_1.CameraTool.getAccurateSize(size, SW, SH);
            scene_tool_1.CameraTool.changeCameraOrth(this.camera, size, SW, SH, sizeForWidth);
        }
        rdPosition(x, y, z) {
            this.camera.position.copyFromFloats(x, y, z);
        }
        rdRotation(x, y, z) {
            this.camera.rotationQuaternion = null;
            this.camera.rotation.copyFromFloats(x, y, z);
        }
    }
    exports.RenderIndexCamera = RenderIndexCamera;
    RenderIndexCamera.pool = new Map();
    RenderIndexCamera.ClassName = render_camera_1.ERenderCameraClassName.indexCamera;
});