_$pi.define("app_c/fight/hp.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./hp.vue.tpl", "app_a/util/new_store", "pi_common/fight/component/common", "pi_utils/util/ecs", "./ecs/fight_init"], function (require, exports, module, direct_1, hp_vue_tpl_1, new_store_1, common_1, ecs_1, fight_init_1) {
    "use strict";

    exports.initMeta = void 0;
    class Hp {
        constructor() {
            this.update = () => {
                const hpC = fight_init_1.world.fetchComponent(ecs_1.Entity, common_1.HpC).get(this.id);
                this.hp = hpC.value;
                this.maxHp = hpC.max;
            };
        }
        create() {
            this.update();
            new_store_1.newStore.register('hp', this.update);
        }
    }
    exports.default = Hp;
    exports.initMeta = () => {
        let _$tpl = "app_c/fight/hp.vue.tpl.ts",
            _$cssPath = "app_c/fight/hp.vue.wcss",
            _$cssHash = 4098735260;
        Hp["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: hp_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Hp, ["hp", "maxHp"]);
    direct_1.addField(Hp, ['id', 'update']);
});