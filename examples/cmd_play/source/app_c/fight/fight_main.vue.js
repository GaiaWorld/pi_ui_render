_$pi.define("app_c/fight/fight_main.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./fight_main.vue.tpl", "./ecs/fight_init", "app_a/util/new_store", "pi_common/ui/main_root"], function (require, exports, module, direct_1, fight_main_vue_tpl_1, fight_init_1, new_store_1, main_root_1) {
    "use strict";

    exports.initMeta = void 0;
    class FightMain {
        constructor() {
            this.list = [];
            this.over = false;
            this.closePage = () => {
                main_root_1.close(this);
                fight_init_1.endFight();
            };
        }
        create() {
            fight_init_1.initWorld();
            new_store_1.newStore.register('inGaming', r => {
                this.list = r;
            });
            new_store_1.newStore.register('gameOver', () => {
                this.over = true;
            });
        }
        start() {
            fight_init_1.startFight();
        }
    }
    exports.default = FightMain;
    exports.initMeta = () => {
        let _$tpl = "app_c/fight/fight_main.vue.tpl.ts",
            _$cssPath = "app_c/fight/fight_main.vue.wcss",
            _$cssHash = 3656213604;
        FightMain["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: fight_main_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(FightMain, ["list", "over"]);
    direct_1.addField(FightMain, ['closePage']);
});