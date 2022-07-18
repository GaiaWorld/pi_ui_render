_$pi.define("app_a/widget/increaser/increaser.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./increaser.vue.tpl", "pi_utils/util/frame_mgr"], function (require, exports, module, direct_1, increaser_vue_tpl_1, frame_mgr_1) {
    "use strict";

    exports.initMeta = void 0;
    class Increaser {
        constructor() {
            this.animator = () => {
                this.interpolation();
            };
        }
        propsUpdate() {
            // 第一次渲染不做任何操作
            if (!this.originNum) {
                this.originNum = this.number;
                return;
            }
            this.destNum = this.number;
            this.number = this.originNum;
            this.playAnimation();
        }
        playAnimation() {
            if (!this.isStartAnimation) {
                this.isStartAnimation = true;
                // 不止一个increaser, 合并animator
                frame_mgr_1.getGlobal().setPermanent(this.animator);
            }
        }
        stopAnimation() {
            frame_mgr_1.getGlobal().clearPermanent(this.animator);
            this.isStartAnimation = false;
        }
        interpolation() {
            const increasingNum = Math.ceil(this.destNum / 60);
            this.number += increasingNum;
            if (this.number >= this.destNum) {
                this.number = this.destNum;
                this.originNum = this.destNum;
                this.stopAnimation();
            }
        }
    }
    exports.default = Increaser;
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/increaser/increaser.vue.tpl.ts",
            _$cssPath = "app_a/widget/increaser/increaser.vue.wcss",
            _$cssHash = 2653555485;
        Increaser["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: increaser_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Increaser, ["fontSize", "color", "number"]);
    direct_1.addField(Increaser, ['originNum', 'destNum', 'isStartAnimation', 'animator']);
});