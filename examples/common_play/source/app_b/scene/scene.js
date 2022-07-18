_$pi.define("app_b/scene/scene", ["require", "exports", "module", "pi_babylon/render3d/babylon", "pi_babylon/scene_tool", "pi_babylon/scene", "app_a/util/bezier", "pi_common/ui/main_root", "pi_gui/engine/format_event"], function (require, exports, module, babylon_1, scene_tool_1, scene_1, bezier_1, main_root_1, format_event_1) {
    "use strict";

    exports.ProjectCustomTool = exports.transEvent = exports.SceneAnimation = void 0;
    var SceneAnimation;
    (function (SceneAnimation) {
        const getCamera = () => scene_1.SceneManagerData.mainscene.camera;
        const deltaSafeCheck = delta => {
            return Math.max(0, Math.min(1, delta));
        };
        function slideRightIn(startTime) {
            const startMoveTime = startTime || Date.now();
            const camera = getCamera();
            const fn = () => {
                const now = Date.now();
                let delta = (now - startMoveTime) / 300;
                delta = deltaSafeCheck(delta);
                let viewportX = bezier_1.easingFunc(delta);
                camera.viewport.x = 1 - viewportX;
                if (viewportX >= 1) {
                    scene_1.SceneManager.unregisterBeforeRenderCall(fn);
                }
            };
            scene_1.SceneManager.registerBeforeRenderCall(fn);
        }
        SceneAnimation.slideRightIn = slideRightIn;
        function slideRightOut(startTime) {
            const startMoveTime = startTime || Date.now();
            const camera = getCamera();
            const fn = () => {
                const now = Date.now();
                let delta = (now - startMoveTime) / 300;
                delta = deltaSafeCheck(delta);
                let viewportX = bezier_1.easingFunc(delta);
                camera.viewport.x = viewportX;
                if (viewportX >= 1) {
                    scene_1.SceneManager.unregisterBeforeRenderCall(fn);
                }
            };
            scene_1.SceneManager.registerBeforeRenderCall(fn);
        }
        SceneAnimation.slideRightOut = slideRightOut;
        function slideLeftOut(startTime) {
            const camera = getCamera();
            const startMoveTime = startTime || Date.now();
            const fn = () => {
                const now = Date.now();
                let delta = (now - startMoveTime) / 300;
                delta = deltaSafeCheck(delta);
                let viewportX = bezier_1.easingFunc(delta);
                camera.viewport.x = -viewportX;
                if (viewportX === 1) {
                    scene_1.SceneManager.unregisterBeforeRenderCall(fn);
                }
            };
            scene_1.SceneManager.registerBeforeRenderCall(fn);
        }
        SceneAnimation.slideLeftOut = slideLeftOut;
        function slideLeftIn(startTime) {
            const startMoveTime = startTime || Date.now();
            const camera = getCamera();
            const fn = () => {
                const now = Date.now();
                let delta = (now - startMoveTime) / 300;
                delta = deltaSafeCheck(delta);
                let viewportX = bezier_1.easingFunc(delta);
                camera.viewport.x = viewportX - 1;
                if (viewportX === 1) {
                    scene_1.SceneManager.unregisterBeforeRenderCall(fn);
                }
            };
            scene_1.SceneManager.registerBeforeRenderCall(fn);
        }
        SceneAnimation.slideLeftIn = slideLeftIn;
    })(SceneAnimation = exports.SceneAnimation || (exports.SceneAnimation = {}));
    exports.transEvent = e => {
        const formatEven = new format_event_1.FormatEvent(null);
        const event = {};
        const vdocument = e.source.document;
        formatEven.recordEventAttr(e.sourceEvent, event);
        event.x = e.sourceEvent.x * vdocument.pixelRatio;
        event.y = e.sourceEvent.y * vdocument.pixelRatio;
        event.clientX = e.sourceEvent.clientX * vdocument.pixelRatio;
        event.clientY = e.sourceEvent.clientY * vdocument.pixelRatio;
        return event;
    };
    exports.ProjectCustomTool = {
        // 将上层事件传递到babylon
        formatEvent(sceneStruct, canvas) {
            var _a, _b, _c;
            const scene = sceneStruct.impl;
            canvas.removeEventListener(`pointermove`, (_a = scene._inputManager) === null || _a === void 0 ? void 0 : _a._onPointerMove);
            canvas.removeEventListener(`pointerdown`, (_b = scene._inputManager) === null || _b === void 0 ? void 0 : _b._onPointerDown);
            window.removeEventListener(`pointerup`, (_c = scene._inputManager) === null || _c === void 0 ? void 0 : _c._onPointerUp);
            const Root = main_root_1.getRoot();
            Root.addDownListener(e => {
                if (!e.isSendNextLayer) return;
                sceneStruct.onPointerDown(exports.transEvent(e));
            });
            Root.addUpListener(e => {
                if (!e.isSendNextLayer) return;
                sceneStruct.onPointerUp(exports.transEvent(e));
            });
            Root.addMoveListener(e => {
                if (!e.isSendNextLayer) return;
                sceneStruct.onPointerMove(exports.transEvent(e));
            });
            Root.addClickListener(e => {
                if (!e.isSendNextLayer) return;
                var trans = exports.transEvent(e);
                sceneStruct.onPointerClick(trans);
            });
        },
        createEffectScene() {
            // 创建特效场景
            const sceneName = 'EffectScene';
            const effectSceneStruct = scene_1.SceneManager.createScene(sceneName);
            effectSceneStruct.impl.autoClear = false;
            effectSceneStruct.impl.ambientColor = babylon_1.BABYLON.Color3.White();
            const cameraName = 'EffectCamera';
            const camera = new babylon_1.BABYLON.FreeCamera(cameraName, new babylon_1.BABYLON.Vector3(0, 0, 0), effectSceneStruct.impl);
            camera.mode = babylon_1.BABYLON.FreeCamera.ORTHOGRAPHIC_CAMERA;
            scene_tool_1.CameraTool.changeCameraOrth(camera, 30, scene_1.SceneManagerData.canvas.width, scene_1.SceneManagerData.canvas.height);
            scene_tool_1.NodeTools.positNode(camera, [0, 0, 5]);
            camera.setTarget(babylon_1.BABYLON.Vector3.Zero());
            // 特效场景去除事件监听（会影响gui接管的事件）
            const scene = effectSceneStruct.impl;
            scene_1.SceneManagerData.canvas.removeEventListener(`pointermove`, scene._onPointerMove);
            scene_1.SceneManagerData.canvas.removeEventListener(`pointerdown`, scene._onPointerDown);
            window.removeEventListener(`pointerup`, scene._onPointerUp);
            effectSceneStruct.addCamera(camera);
            effectSceneStruct.setCurrCamera(camera.name);
            return effectSceneStruct;
        }
    };
});