_$pi.define("app_b/scene/scene_data", ["require", "exports", "module", "pi_utils/util/event"], function (require, exports, module, event_1) {
    "use strict";

    exports.MODEL_SCALE = exports.ALPHA_INDEX = exports.IdManager = exports.bus = exports.CALL_FLAG = exports.MODEL_TYPE = exports.SceneEvent = exports.SceneData = exports.MAP_LIMIT = void 0;
    /**
     * 场景基础数据定义
     */
    exports.MAP_LIMIT = [2.45, -2.45, -8, 8]; // 地图边缘限制 [左 右 上 下]
    // 场景名
    exports.SceneData = {
        mainSceneName: 'MAIN_SCENE',
        animSceneName: 'ANIM_SCENE',
        shadowGenerator: null,
        syntSceneName: 'SYNT_SCENE',
        effectSceneName: 'EFFECT_SCENE',
        lightSceneName: 'LIGHT_SCENE'
    };
    // 场景事件
    var SceneEvent;
    (function (SceneEvent) {
        SceneEvent["SCENE_LOADED"] = "SCENE_LOADED";
    })(SceneEvent = exports.SceneEvent || (exports.SceneEvent = {}));
    // model类型
    var MODEL_TYPE;
    (function (MODEL_TYPE) {
        MODEL_TYPE[MODEL_TYPE["ANIMAL"] = 1] = "ANIMAL";
        MODEL_TYPE[MODEL_TYPE["BUILDING"] = 2] = "BUILDING";
        MODEL_TYPE[MODEL_TYPE["VISITOR"] = 3] = "VISITOR";
        MODEL_TYPE[MODEL_TYPE["OTHER"] = 4] = "OTHER";
        MODEL_TYPE[MODEL_TYPE["BILLBORAD"] = 5] = "BILLBORAD";
        MODEL_TYPE[MODEL_TYPE["NPC"] = 6] = "NPC"; // 答题npc
    })(MODEL_TYPE = exports.MODEL_TYPE || (exports.MODEL_TYPE = {}));
    //
    var CALL_FLAG;
    (function (CALL_FLAG) {
        CALL_FLAG[CALL_FLAG["FLAG_1"] = 1] = "FLAG_1";
        CALL_FLAG[CALL_FLAG["FLAG_2"] = 2] = "FLAG_2";
        CALL_FLAG[CALL_FLAG["FLAG_3"] = 3] = "FLAG_3";
    })(CALL_FLAG = exports.CALL_FLAG || (exports.CALL_FLAG = {}));
    // 主场景事件总线
    exports.bus = new event_1.HandlerMap();
    // id管理器
    exports.IdManager = {
        i: 1,
        getId() {
            return this.i++ << 24;
        }
    };
    // 渲染顺序
    exports.ALPHA_INDEX = {
        BG: 2500,
        MASK: 2500,
        DECORATION: 2600,
        DECORATION_TIPS: 2601
    };
    // 模型缩放
    exports.MODEL_SCALE = [1, 1, 1];
});