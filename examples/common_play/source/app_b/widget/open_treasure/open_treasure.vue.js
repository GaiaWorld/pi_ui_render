_$pi.define("app_b/widget/open_treasure/open_treasure.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./open_treasure.vue.tpl", "../../scene/effect", "pi_utils/util/frame_mgr", "pi_common/ui/main_root"], function (require, exports, module, direct_1, open_treasure_vue_tpl_1, effect_1, frame_mgr_1, main_root_1) {
    "use strict";

    exports.initMeta = exports.setInsliding = void 0;
    exports.setInsliding = flag => {
        OpenTreasure.insliding = flag;
    };
    class OpenTreasure {
        constructor() {
            this.boxType = 2;
            this.rest = 3;
            this.closePage = () => {
                if (!this.showTip) {
                    return;
                }
                effect_1.EffectScene.dispose(effect_1.EffectScene.EffectModelId.BoxFall);
                effect_1.EffectScene.dispose(effect_1.EffectScene.EffectModelId.BoxOpen);
                effect_1.EffectScene.dispose(effect_1.EffectScene.EffectModelId.BoxStand);
                main_root_1.close(this);
            };
            this.boxFallAnime = () => {
                OpenTreasure.insliding = true;
                if (!this.animationContraller) {
                    this.animationContraller = effect_1.EffectScene.getBoxAnimalControl(this.boxType);
                }
                this.animationContraller.setTopCall(() => {
                    this.awradAnime();
                });
                frame_mgr_1.getGlobal().setAfter(() => this.animationContraller.play(effect_1.EffectScene.BoxAnimation.FALL, () => {
                    OpenTreasure.insliding = false;
                    this.animationContraller.setNum(this.rest);
                }));
            };
            this.boxOpenAnime = () => {
                if (this.rest <= 0) {
                    this.showTip = true;
                    return this.closePage();
                }
                OpenTreasure.insliding = true;
                this.rest--;
                this.animationContraller.setNum(this.rest);
                this.animationContraller.play(effect_1.EffectScene.BoxAnimation.OPEN);
            };
            this.awradAnime = () => {
                if (this.rest <= 0) {
                    this.showTip = true;
                }
            };
        }
        attach() {
            this.boxFallAnime();
        }
    }
    exports.default = OpenTreasure;
    exports.initMeta = () => {
        let _$tpl = "app_b/widget/open_treasure/open_treasure.vue.tpl.ts",
            _$cssPath = "app_b/widget/open_treasure/open_treasure.vue.wcss",
            _$cssHash = 1742907318;
        OpenTreasure["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: open_treasure_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(OpenTreasure, ["showTip"]);
    direct_1.addField(OpenTreasure, ['ok', 'boxType', 'rest', 'closePage', 'boxFallAnime', 'boxOpenAnime', 'awradAnime']);
});