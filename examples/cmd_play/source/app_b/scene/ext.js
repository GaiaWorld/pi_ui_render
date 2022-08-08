_$pi.define("app_b/scene/ext", ["require", "exports", "module", "./scene_data", "./utils", "pi_babylon/scene"], function (require, exports, module, scene_data_1, utils_1, scene_1) {
    "use strict";

    exports.ModelManager = exports.Ext = exports.MoveableModel = exports.Model = void 0;
    const modelReg = /^mod_poly_(\d*)$/;
    // 基础模型父类，提供默认模型加载及回调设置
    class Model {
        constructor(opts) {
            this.loadCalls = [];
            this.opts = opts;
            this.rayId = opts.rayId;
            this._position = opts.position;
            this.modelName = opts.modelName;
            this.path = opts.path || opts.modelName;
            if (modelReg.test(opts.modelName)) {
                this.animationKey = opts.modelName.match(modelReg)[1];
            }
            this.init(opts.isEffect);
            this.setup();
        }
        get position() {
            return this._position;
        }
        set position(data) {
            if (!data) return;
            this._position = data;
            this.model.setPostion(data);
        }
        get avatar() {
            return this._avatar || this.model.rootImpl;
        }
        set avatar(value) {
            this._avatar = value;
        }
        init(isEffect) {
            const id = scene_data_1.IdManager.getId();
            const impl = utils_1.getMainScene();
            const modelName = this.modelName;
            const loadOpts = {
                isEffect,
                path: `${this.path}/`,
                fileName: modelName,
                rayID: this.rayId,
                particleAutoStart: !!this.opts.particleAutoStart,
                insertedCall: model => {
                    model.rootImpl.getChildMeshes().forEach(mesh => {
                        mesh.isPickable = true;
                        mesh.rayID = this.rayId;
                    });
                    this.processFuncs();
                }
            };
            this.opts.imageSolts && (loadOpts.imageSolts = this.opts.imageSolts);
            this.model = impl.insertMesh(`${modelName}${id}`, loadOpts);
            this.position = this._position;
        }
        processFuncs() {
            while (this.loadCalls.length) {
                const fn = this.loadCalls.pop();
                fn && fn();
            }
        }
        execCall(fn) {
            if (this.model && this.model.isLoaded) {
                fn();
            } else {
                this.loadCalls.push(fn);
            }
        }
        /**
         * 获取模型位置
         * @param y 模型中心点美术制作在底部，为居中0.6为粗略估计,可调整
         */
        getScreenCoord(y) {
            if (!this.model.isLoaded) throw new Error('no model!!!!');
            const position = this.model.rootImpl.getChildMeshes()[0].getAbsolutePosition();
            const coord = utils_1.sceneCoordConvert(position.x, y || 0.6, position.z);
            return coord;
        }
        // --implemented by children--
        setup() {}
        update() {}
        onClick(_triggerType) {}
        onExtCall(_tag, _value) {}
    }
    exports.Model = Model;
    /**
     * 可移动模型，例如动物，游客，npc等
     */
    class MoveableModel extends Model {
        // 设置下一个目标点
        setTargetPoint(value) {
            const [x,, z] = this.position;
            const [ex,, ez] = value;
            if (x == ex && z == ez) return;
            this.targetPoint = value;
            this.pause = false;
            const rotate = utils_1.calcRotate(x, z, ex, ez);
            if (rotate) this.model.setRotate([0, rotate - 3.14 / 2, 0]);
            this.moveDuration = utils_1.calcMoveTime(x, z, ex, ez);
            this.lastPosition = this.position.slice();
            this.actionWalk();
            this.pause = false;
        }
        // 移动
        move() {
            if (!this.targetPoint) return;
            const now = Date.now();
            if (!this.startMoveTime) this.startMoveTime = now;
            let dt = now - this.startMoveTime;
            const [ex,, ez] = this.targetPoint;
            const [lx,, lz] = this.lastPosition;
            dt = dt >= this.moveDuration ? this.moveDuration : dt;
            const moveDelta = dt / this.moveDuration;
            const mx = (ex - lx) * moveDelta;
            const my = (ez - lz) * moveDelta;
            this.position = [lx + mx, 0, lz + my];
            if (this.isArrived()) {
                this.position = this.targetPoint;
                this.arrivedHandle();
            }
        }
        // 子类覆写
        arrivedHandle() {}
        isArrived() {
            if (!this.targetPoint) return;
            const [px,, pz] = this.position;
            const [ax,, az] = this.targetPoint;
            return px.toFixed(2) == ax.toFixed(2) && pz.toFixed(2) == az.toFixed(2);
        }
        actionStandby() {
            this.model.setAnim({
                animName: `act_${this.animationKey}_idle`,
                isLoop: true
            });
        }
        actionHappy(call) {
            this.model.setAnim({
                animName: `act_${this.animationKey}_happy`,
                isLoop: false,
                endCall: call
            });
        }
        actionWalk() {
            this.model.setAnim({
                animName: `act_${this.animationKey}_walk`,
                isLoop: true
            });
        }
        actionShow(endCall) {
            this.model.setAnim({
                animName: `act_${this.animationKey}_show`,
                isLoop: false,
                endCall
            });
        }
    }
    exports.MoveableModel = MoveableModel;
    class Ext extends Model {
        constructor(opts) {
            opts.isEffect = true;
            super(opts);
        }
        // 设置父节点
        setParent(parent) {
            this.parent = parent;
            let connectFun = this.connect.bind(this);
            parent.execCall(connectFun);
        }
        connect() {
            if (this.model) {
                this.execCall(() => {
                    this.model.rootImpl.parent = this.parent.avatar;
                    this.onAttached();
                });
            } else {
                this.mesh.parent = this.parent.avatar;
                this.onAttached();
            }
        }
        // 当附加到parent时触发
        onAttached() {}
    }
    exports.Ext = Ext;
    // 模型管理
    var ModelManager;
    (function (ModelManager) {
        let _modelMap = new Map();
        let _pause;
        function getAllModels() {
            return _modelMap;
        }
        ModelManager.getAllModels = getAllModels;
        function add(model) {
            _modelMap.set(model.rayId, model);
        }
        ModelManager.add = add;
        function remove(model) {
            _modelMap.delete(model.rayId);
        }
        ModelManager.remove = remove;
        function removeAll() {
            _modelMap.forEach(model => {
                if (model.modelType == scene_data_1.MODEL_TYPE.VISITOR || model.modelType == scene_data_1.MODEL_TYPE.OTHER) return;
                model.dispose();
                ModelManager.remove(model);
            });
        }
        ModelManager.removeAll = removeAll;
        function removeAnimal() {
            _modelMap.forEach(model => {
                if (model.modelType == scene_data_1.MODEL_TYPE.ANIMAL) {
                    model.dispose();
                    ModelManager.remove(model);
                }
            });
        }
        ModelManager.removeAnimal = removeAnimal;
        function removeBuilding() {
            _modelMap.forEach(model => {
                if (model.modelType == scene_data_1.MODEL_TYPE.BUILDING) {
                    model.dispose();
                    ModelManager.remove(model);
                }
            });
        }
        ModelManager.removeBuilding = removeBuilding;
        function getModelById(rayId) {
            return _modelMap.get(rayId);
        }
        ModelManager.getModelById = getModelById;
        function getAnimalIds() {
            let _array = [];
            _modelMap.forEach(model => {
                if (model.modelType === scene_data_1.MODEL_TYPE.ANIMAL) _array.push(model.rayId);
            });
            return _array;
        }
        ModelManager.getAnimalIds = getAnimalIds;
        function renderLoop() {
            if (_pause) return;
            _modelMap.forEach(model => {
                model === null || model === void 0 ? void 0 : model.update();
            });
        }
        ModelManager.renderLoop = renderLoop;
        function pause() {
            _pause = true;
        }
        ModelManager.pause = pause;
        function active() {
            _pause = false;
        }
        ModelManager.active = active;
        // 加载时即注册帧调用，驱动场景更新(动物行走，收益变化等)
        scene_1.SceneManager.registerBeforeRenderCall(renderLoop);
    })(ModelManager = exports.ModelManager || (exports.ModelManager = {}));
});