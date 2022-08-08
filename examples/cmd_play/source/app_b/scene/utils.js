_$pi.define("app_b/scene/utils", ["require", "exports", "module", "pi_babylon/render3d/babylon", "./scene_data", "pi_babylon/scene", "pi_common/ui/main_root"], function (require, exports, module, babylon_1, scene_data_1, scene_1, main_root_1) {
    "use strict";

    exports.calcMoveTime = exports.getSensibility = exports.createTextPlane = exports.createLoopMoveAnimation = exports.createScaleAnimation = exports.createAlphaAnimation = exports.createPutAnmiation = exports.createMoveAnimation = exports.createCoinWaveAnimation = exports.calcRotate = exports.sceneCoordConvert = exports.shuffle = exports.getTextureByName = exports.ensureCameraMove = exports.pnpoly = exports.getVertextArray = exports.getMainScene = exports.isEmpty = void 0;
    exports.isEmpty = obj => {
        if (obj instanceof Array) {
            return obj.length === 0;
        } else return Object.keys(obj).length === 0;
    };
    /**
     * 场景工具函数库
     */
    exports.getMainScene = () => scene_1.SceneManagerData.sceneMap.get(scene_data_1.SceneData.mainSceneName);
    // 获取分组顶点供pnpoly使用
    exports.getVertextArray = polys => {
        return polys.reduce((pre, curr) => {
            (pre[0] || (pre[0] = [])).push(curr[0]);
            (pre[1] || (pre[1] = [])).push(curr[1]);
            return pre;
        }, []);
    };
    /**
     * PNPoly算法 https://wrf.ecse.rpi.edu//Research/Short_Notes/pnpoly.html
     * @param nvert 多边形有几个点
     * @param vertx 储存多边形横坐标数组
     * @param verty 储存多边形纵坐标数组
     * @param testx 待测试点的横坐标
     * @param testy 待测试点的纵坐标
     */
    function pnpoly(nvert, vertx, verty, testx, testy) {
        let i,
            j,
            c = 0;
        // tslint:disable-next-line:ban-comma-operator
        for (i = 0, j = nvert - 1; i < nvert; j = i++) {
            if (verty[i] > testy !== verty[j] > testy && testx < (vertx[j] - vertx[i]) * (testy - verty[i]) / (verty[j] - verty[i]) + vertx[i]) {
                c = !c;
            }
        }
        return !!c;
    }
    exports.pnpoly = pnpoly;
    // 检查相机移动限制
    exports.ensureCameraMove = (camera, moveX, moveY) => {
        const p = camera.position;
        const offsetR = camera.orthoRight;
        const offsetT = camera.orthoTop * 2;
        const centerX = -p.x,
              centerZ = -p.z - 20;
        if (centerX + moveX + offsetR > scene_data_1.MAP_LIMIT[0] || centerX + moveX - offsetR < scene_data_1.MAP_LIMIT[1] || centerZ - moveY - offsetT < scene_data_1.MAP_LIMIT[2] || centerZ - moveY + offsetT > scene_data_1.MAP_LIMIT[3]) return false;
        return true;
    };
    const _textureCatch = new Map();
    // 根据文件名创建贴图
    exports.getTextureByName = (fileName, suffix, callBack) => {
        const sceneStruct = exports.getMainScene();
        if (_textureCatch.has(fileName)) {
            callBack && callBack();
            return _textureCatch.get(fileName).clone();
        }
        const url = `app/scene_res/res/images/${fileName}.${suffix ? suffix : 'png'}`;
        const texture = new babylon_1.BABYLON.Texture(url, sceneStruct.impl);
        texture.onLoadObservable.addOnce(() => {
            callBack && callBack();
        });
        texture.hasAlpha = true;
        _textureCatch.set(fileName, texture);
        return texture.clone();
    };
    // 洗牌算法，会改变原数组
    function shuffle(arr) {
        var k = 0;
        var temp = 0;
        for (var i = 0; i < arr.length; i++) {
            k = Math.floor(Math.random() * (arr.length - i));
            temp = arr[i];
            arr[i] = arr[k];
            arr[k] = temp;
        }
    }
    exports.shuffle = shuffle;
    exports.sceneCoordConvert = (x, y, z, struct) => {
        const sceneStruct = struct || exports.getMainScene();
        const scene = sceneStruct.impl,
              camera = scene.activeCamera;
        const vec3 = new babylon_1.BABYLON.Vector3(x, y, z);
        let root = main_root_1.getRoot();
        const coord = babylon_1.BABYLON.Vector3.Project(vec3, babylon_1.BABYLON.Matrix.IdentityReadOnly, scene.getTransformMatrix(), camera.viewport.toGlobal(root.document.width, root.document.height));
        return coord;
    };
    exports.calcRotate = (x, y, x1, y1) => {
        const dx = x - x1,
              dy = y - y1;
        if (x === x1 && y < y1) return 3.14;
        if (x === x1 && y > y1) return 0.01 / 180 * 3.14;
        if (x > x1 && y === y1) return -3.14 / 2;
        if (x < x1 && y === y1) return 3.14 / 2;
        return -Math.atan2(dy, dx);
    };
    // wave 动画
    exports.createCoinWaveAnimation = (animName, target) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'scaling', 300, babylon_1.BABYLON.Animation.ANIMATIONTYPE_VECTOR3, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);
        const keys = [{
            frame: 0,
            value: new babylon_1.BABYLON.Vector3(0.65, 0.65, 0.65)
        }, {
            frame: 25,
            value: new babylon_1.BABYLON.Vector3(0.75, 0.75, 0.75)
        }, {
            frame: 50,
            value: new babylon_1.BABYLON.Vector3(0.7, 0.7, 0.7)
        }, {
            frame: 75,
            value: new babylon_1.BABYLON.Vector3(0.65, 0.65, 0.65)
        }, {
            frame: 100,
            value: new babylon_1.BABYLON.Vector3(0.7, 0.7, 0.7)
        }];
        animation.setKeys(keys);
        const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
        ag.addTargetedAnimation(animation, target);
        return ag;
    };
    exports.createMoveAnimation = (animName, target, start, end, framePerSecond, totalFrame, createAnimationGroup = true) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'position', framePerSecond, babylon_1.BABYLON.Animation.ANIMATIONTYPE_VECTOR3, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);
        const keys = [{
            frame: 0,
            value: start
        }, {
            frame: totalFrame,
            value: end
        }];
        animation.setKeys(keys);
        let result;
        if (createAnimationGroup) {
            const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
            ag.addTargetedAnimation(animation, target);
            ag.onAnimationEndObservable.add(() => {
                ag.dispose();
            });
            result = ag;
        } else {
            result = animation;
        }
        return result;
    };
    // 缩放
    exports.createPutAnmiation = (animName, target) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'scaling', 50, babylon_1.BABYLON.Animation.ANIMATIONTYPE_VECTOR3, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);
        const keys = [{
            frame: 0,
            value: new babylon_1.BABYLON.Vector3(1, 1, 1)
        }, {
            frame: 5,
            value: new babylon_1.BABYLON.Vector3(1.1, 1.1, 1.1)
        }, {
            frame: 10,
            value: new babylon_1.BABYLON.Vector3(0.9, 0.9, 0.9)
        }, {
            frame: 15,
            value: new babylon_1.BABYLON.Vector3(1.05, 1.05, 1.05)
        }, {
            frame: 20,
            value: new babylon_1.BABYLON.Vector3(1, 1, 1)
        }];
        animation.setKeys(keys);
        const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
        ag.addTargetedAnimation(animation, target);
        ag.onAnimationEndObservable.add(() => {
            ag.dispose();
        });
        return ag;
    };
    exports.createAlphaAnimation = (animName, target, framePerSecond, totalFrame, from, to, createAnimationGroup = true) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'visibility', framePerSecond, babylon_1.BABYLON.Animation.ANIMATIONTYPE_FLOAT, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);
        const keys = [{
            frame: 0,
            value: from
        }, {
            frame: totalFrame,
            value: to
        }];
        animation.setKeys(keys);
        let result;
        if (createAnimationGroup) {
            const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
            ag.addTargetedAnimation(animation, target);
            ag.onAnimationEndObservable.add(() => {
                ag.dispose();
            });
            result = ag;
        } else {
            result = animation;
        }
        return result;
    };
    exports.createScaleAnimation = (animName, target, framePerSecond, totalFrame, from, to, createAnimationGroup = true) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'scaling', framePerSecond, babylon_1.BABYLON.Animation.ANIMATIONTYPE_VECTOR3, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CONSTANT);
        const keys = [{
            frame: 0,
            value: from
        }, {
            frame: totalFrame,
            value: to
        }];
        animation.setKeys(keys);
        let result;
        if (createAnimationGroup) {
            const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
            ag.addTargetedAnimation(animation, target);
            ag.onAnimationEndObservable.add(() => {
                ag.dispose();
            });
            result = ag;
        } else {
            result = animation;
        }
        return result;
    };
    exports.createLoopMoveAnimation = (animName, target, start, end, framePerSecond, totalFrame, createAnimationGroup = true) => {
        const sceneStruct = exports.getMainScene();
        const animation = new babylon_1.BABYLON.Animation(animName, 'position.y', framePerSecond, babylon_1.BABYLON.Animation.ANIMATIONTYPE_FLOAT, babylon_1.BABYLON.Animation.ANIMATIONLOOPMODE_CYCLE);
        const keys = [{
            frame: 0,
            value: start
        }, {
            frame: totalFrame / 4,
            value: end
        }, {
            frame: totalFrame / 2,
            value: start
        }, {
            frame: totalFrame / 4 * 3,
            value: end
        }, {
            frame: totalFrame,
            value: start
        }];
        animation.setKeys(keys);
        let result;
        if (createAnimationGroup) {
            const ag = new babylon_1.BABYLON.AnimationGroup(`${animName}AnimationGroup`, sceneStruct.impl);
            ag.addTargetedAnimation(animation, target);
            result = ag;
        } else {
            result = animation;
        }
        return result;
    };
    // 创建文字
    let tmpctx;
    exports.createTextPlane = opts => {
        const scene = opts.scene || exports.getMainScene().impl;
        const font_size = opts.fontSize || 24;
        const font = `${font_size}px Roboto`;
        const planeHeight = 1.5;
        const DTHeight = 1.1 * font_size;
        const ratio = planeHeight / DTHeight;
        const text = ` ${opts.tips} `;
        const [strokeOffsetX, strokeOffsetY] = opts.strokeOffset || [0, 0];
        if (!tmpctx) {
            const canvas = document.createElement('canvas');
            canvas.width = 1;
            canvas.height = 1;
            tmpctx = canvas.getContext('2d');
        }
        tmpctx.font = font;
        const DTWidth = tmpctx.measureText(text).width;
        const planeWidth = DTWidth * ratio;
        const texture = new babylon_1.BABYLON.DynamicTexture(`moneyTexture${scene_data_1.IdManager.getId()}`, { width: DTWidth, height: DTHeight }, scene, false);
        const size = texture.getSize();
        const ctx = texture.getContext(),
              clearColor = 'transparent',
              color = opts.textColor || '#fff';
        let x = null,
            y = null;
        if (clearColor) {
            ctx.fillStyle = clearColor;
            ctx.fillRect(0, 0, size.width, size.height);
        }
        ctx.font = font;
        ctx.lineWidth = opts.lineWidth || 3;
        if (x === null || x === undefined) {
            const textSize = ctx.measureText(text);
            x = (size.width - textSize.width) / 2;
        }
        if (y === null || y === undefined) {
            // tslint:disable-next-line:radix
            const fontSize = parseInt(font.replace(/\D/g, ''));
            y = size.height / 2 + fontSize / 3.65;
        }
        ctx.strokeStyle = opts.strokeColor || '#4a2111';
        ctx.strokeText(text, x + strokeOffsetX, y + strokeOffsetY);
        ctx.fillStyle = color;
        ctx.fillText(text, x, y);
        texture.update(false);
        const textMaterial = new babylon_1.BABYLON.StandardMaterial(`moneyTextureMaterial${scene_data_1.IdManager.getId()}`, scene);
        textMaterial.diffuseTexture = texture;
        textMaterial.diffuseTexture.hasAlpha = true;
        textMaterial.useAlphaFromDiffuseTexture = true;
        textMaterial.emissiveColor = babylon_1.BABYLON.Color3.White();
        textMaterial.disableLighting = true;
        const plane = babylon_1.BABYLON.MeshBuilder.CreatePlane(`moneyPlane${scene_data_1.IdManager.getId()}`, { width: planeWidth, height: planeHeight }, scene);
        plane.material = textMaterial;
        plane.bakeCurrentTransformIntoVertices();
        plane.scaling = new babylon_1.BABYLON.Vector3(-0.25, 0.25, 1);
        return { plane, planeWidth, planeHeight };
    };
    exports.getSensibility = camera => {
        const orthoTop = camera.orthoTop,
              orthoRight = camera.orthoRight;
        const vh = scene_1.SceneManagerData.canvas.height,
              vw = scene_1.SceneManagerData.canvas.width;
        const zSensibility = vh / (orthoTop * 4);
        const xSensibility = vw / (orthoRight * 2);
        return { zSensibility, xSensibility };
    };
    // 每秒移动场景单位
    const MOVE_SPEED = 1;
    // 计算移动所需花费时间
    exports.calcMoveTime = (x, y, x1, y1) => {
        return Math.round(Math.sqrt(Math.pow(x - x1, 2) + Math.pow(y - y1, 2)) / MOVE_SPEED * 1000);
    };
});