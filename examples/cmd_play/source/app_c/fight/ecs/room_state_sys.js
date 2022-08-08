_$pi.define("app_c/fight/ecs/room_state_sys", ["require", "exports", "module", "app_a/util/new_store", "pi_common/fight/component/common", "pi_common/fight/component/round", "pi_common/fight/component/skill", "pi_common/fight/fight_init", "pi_common/fight/single/global", "pi_common/fight/single/round", "pi_common/scene/single/global", "pi_utils/util/ecs", "./fight_init", "./room_state"], function (require, exports, module, new_store_1, common_1, round_1, skill_1, fight_init_1, global_1, round_2, global_2, ecs_1, fight_init_2, room_state_1) {
    "use strict";

    exports.RoomStateSys = void 0;
    class RoomStateSys {
        constructor() {
            this.count = 0;
        }
        // 监听角色创建
        listenDataC(e, _read, write) {
            console.log('==================== listenDataC', e.id);
            const roomState = write[0].data;
            roomState.ids.push(e.id);
            if (roomState.ids.length === 2) {
                roomState.state = room_state_1.RoomStateEnum.IN_GAMEING;
            }
        }
        // 监听血量变化
        listenHpc(e, read, write) {
            const roomState = write[0].data;
            const hpC = read[0];
            const hp = hpC.get(e.id).value;
            if (roomState.state !== room_state_1.RoomStateEnum.IN_GAMEING) return;
            console.log('================== listenHpc', e.id, hp);
            new_store_1.newStore.notify('hp', hp);
            if (hp <= 0) {
                roomState.state = room_state_1.RoomStateEnum.GAME_OVER;
            }
        }
        // 监听房间状态
        listenRoomState(e, read, write) {
            const roomState = read[0].data;
            const frame = write[0].data;
            console.log('================= listenRoomState', room_state_1.RoomStateEnum[roomState.state]);
            if (roomState.state === room_state_1.RoomStateEnum.WAIT_START) {
                fight_init_2.addFighter();
            } else if (roomState.state === room_state_1.RoomStateEnum.IN_GAMEING) {
                fight_init_1.startRoundFight(fight_init_2.world);
                new_store_1.newStore.notify('inGaming', roomState.ids);
            } else if (roomState.state === room_state_1.RoomStateEnum.GAME_OVER) {
                // 游戏结束 让定时器停止
                frame.count = frame.maxCount;
                new_store_1.newStore.notify('gameOver', roomState.ids);
            }
        }
        // 监听全局回合变化
        listenGlobalRound(_e, read, write) {
            const round = read[0].data.count;
            const timeout = write[1].data;
            console.log('################### GlobalRound change round ', round);
            if (round > 5) {
                write[0].data.state = room_state_1.RoomStateEnum.GAME_OVER;
                return;
            }
            let time = 1;
            for (const [id, v] of read[1].iter()) {
                time++;
                timeout.value = new global_2.Timer(time * 1000, () => {
                    this.fight(id);
                });
            }
        }
        listenRoundCountC(e, read, write) {
            const round = read[0].data.count;
            for (const [id, v] of read[1].iter()) {
                if (v.value < round) return;
            }
            write[0].data.round = round;
        }
        fight(id) {
            this.count++;
            const roundState = fight_init_2.world.fetchComponent(ecs_1.Entity, round_1.RoundStateC);
            const roundOrder = fight_init_2.world.fetchComponent(ecs_1.Entity, round_1.RoundOrderC);
            roundOrder.get(id).value = this.count;
            roundState.get(id).value = 0;
            const ext = fight_init_2.world.fetchSingle(global_1.Ext).data;
            const skillCmd = fight_init_2.world.fetchComponent(ecs_1.Entity, skill_1.SkillCmdC).get(id);
            const cmds = ext.autoSkill(id);
            for (const v of cmds) {
                skillCmd.value = v.value;
            }
        }
        setup() {}
    }
    __decorate([ecs_1.listenCreate([ecs_1.Entity, room_state_1.DataC]), ecs_1.read([ecs_1.Entity, room_state_1.DataC]), ecs_1.write(room_state_1.RoomState)], RoomStateSys.prototype, "listenDataC", null);
    __decorate([ecs_1.listenModify([ecs_1.Entity, common_1.HpC]), ecs_1.read([ecs_1.Entity, common_1.HpC]), ecs_1.write(room_state_1.RoomState)], RoomStateSys.prototype, "listenHpc", null);
    __decorate([ecs_1.listenModify(room_state_1.RoomState), ecs_1.read(room_state_1.RoomState), ecs_1.write(global_2.FrameTime)], RoomStateSys.prototype, "listenRoomState", null);
    __decorate([ecs_1.listenModify(round_2.GlobalRound), ecs_1.read(round_2.GlobalRound, [ecs_1.Entity, room_state_1.DataC]), ecs_1.write(room_state_1.RoomState, global_2.Timeout)], RoomStateSys.prototype, "listenGlobalRound", null);
    __decorate([ecs_1.listenModify([ecs_1.Entity, round_1.RoundCountC]), ecs_1.read(round_2.GlobalRound, [ecs_1.Entity, round_1.RoundCountC]), ecs_1.write(round_2.GlobalReady)], RoomStateSys.prototype, "listenRoundCountC", null);
    exports.RoomStateSys = RoomStateSys;
});