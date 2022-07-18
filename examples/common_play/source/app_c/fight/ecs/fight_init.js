_$pi.define("app_c/fight/ecs/fight_init", ["require", "exports", "module", "app_a/util/cfg_map", "app_c/fight/ecs/round_skill/buffModel.struct", "app_c/fight/ecs/round_skill/skillModel.struct", "pi_common/fight/component/common", "pi_common/fight/component/skill", "pi_common/fight/single/global", "pi_common/fight/system/add_sys", "pi_common/fight/util/common_util", "pi_common/fight/util/init_util", "pi_common/fight/util/round_util", "pi_common/scene/single/global", "pi_utils/util/ecs", "./room_state", "./room_state_sys"], function (require, exports, module, cfg_map_1, buffModel_struct_1, skillModel_struct_1, common_1, skill_1, global_1, add_sys_1, common_util_1, init_util_1, round_util_1, global_2, ecs_1, room_state_1, room_state_sys_1) {
    "use strict";

    exports.AddFighterSys = exports.addFighter = exports.endFight = exports.startFight = exports.initWorld = exports.world = void 0;
    window.world = exports.world;
    // 初始化
    exports.initWorld = () => {
        exports.world = init_util_1.initFight();
        round_util_1.initRound(exports.world, false, false, false);
        common_util_1.initHp(exports.world);
        common_util_1.initCampPosition(exports.world);
        exports.world.registerComponent(ecs_1.Entity, room_state_1.DataC, "data");
        exports.world.registerSingle(new global_2.AddCmd(), 'AddCmd');
        exports.world.registerSingle(new room_state_1.RoomState(), 'RoomState');
        // 注册添加战斗者的系统
        exports.world.registerSystem("AddFighterSys", new AddFighterSys());
        exports.world.registerSystem('RoomStateSys', new room_state_sys_1.RoomStateSys());
        const skillModel = cfg_map_1.getMap(skillModel_struct_1.SkillModel);
        const buffModel = cfg_map_1.getMap(buffModel_struct_1.BuffModel);
        exports.world.fetchSingle(global_1.SkillLib).data.map = skillModel;
        exports.world.fetchSingle(global_1.BuffLib).data.map = buffModel;
        const ext = exports.world.fetchSingle(global_1.Ext).data;
        const libC = exports.world.fetchComponent(ecs_1.Entity, skill_1.SkillLibC);
        ext.autoSkill = fighter => {
            let lib = libC.get(fighter);
            // 默认使用第一个可以使用的技能
            for (let r of lib.map.values()) {
                if (ext.isCooldown(fighter, r.nextTime) || r.get("passive")) {
                    continue;
                }
                let cmd = new skill_1.SkillCmdC();
                cmd.value = new skill_1.SkillCmd(r.id);
                return [cmd];
            }
        };
        // 绑定治疗效果
        ext["addHP"] = common_util_1.heal(exports.world.fetchComponent(ecs_1.Entity, common_1.HpC));
    };
    // 开始战斗
    exports.startFight = () => {
        const roomState = exports.world.fetchSingle(room_state_1.RoomState).data;
        roomState.ids = [];
        roomState.state = room_state_1.RoomStateEnum.WAIT_START;
    };
    // 结束战斗
    exports.endFight = () => {
        const roomState = exports.world.fetchSingle(room_state_1.RoomState).data;
        roomState.state = room_state_1.RoomStateEnum.GAME_OVER;
        const entity = exports.world.fetchEntity(ecs_1.Entity);
        for (const id of entity.iter()) {
            exports.world.destroyEntity(ecs_1.Entity, id);
        }
    };
    // 添加战斗者
    exports.addFighter = () => {
        const add = exports.world.fetchSingle(global_2.AddCmd).data;
        const list = [{
            hp: 100,
            maxHp: 100,
            camp: 1,
            skills: [1],
            position: [0, 0],
            data: new room_state_1.DataC(1)
        }, {
            hp: 100,
            maxHp: 100,
            camp: 2,
            skills: [6],
            position: [0, 0],
            data: new room_state_1.DataC(2)
        }];
        add.value = list;
    };
    // 添加战斗者的系统
    class AddFighterSys extends add_sys_1.FighterAddSys {
        setup(world) {
            super.setup(world);
            this.dataC = world.fetchComponent(ecs_1.Entity, room_state_1.DataC);
        }
        init_entity(world, id, cmd) {
            super.init_entity(world, id, cmd);
            this.dataC.insert(id, cmd.data);
        }
    }
    exports.AddFighterSys = AddFighterSys;
});