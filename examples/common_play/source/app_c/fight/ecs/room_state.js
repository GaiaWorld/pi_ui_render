_$pi.define("app_c/fight/ecs/room_state", ["require", "exports", "module", "pi_utils/util/ecs"], function (require, exports, module, ecs_1) {
    "use strict";

    exports.DataC = exports.RoomState = exports.RoomStateEnum = void 0;
    // 房间状态
    var RoomStateEnum;
    (function (RoomStateEnum) {
        RoomStateEnum[RoomStateEnum["WAIT_START"] = 1] = "WAIT_START";
        RoomStateEnum[RoomStateEnum["IN_GAMEING"] = 2] = "IN_GAMEING";
        RoomStateEnum[RoomStateEnum["GAME_OVER"] = 3] = "GAME_OVER";
    })(RoomStateEnum = exports.RoomStateEnum || (exports.RoomStateEnum = {}));
    class RoomState {
        constructor() {
            this.ids = [];
        }
    }
    __decorate([ecs_1.writeNotify], RoomState.prototype, "state", void 0);
    exports.RoomState = RoomState;
    // 定义自由属性组件
    class DataC extends ecs_1.Component {
        constructor(id) {
            super();
            this.id = id;
        }
    }
    exports.DataC = DataC;
});