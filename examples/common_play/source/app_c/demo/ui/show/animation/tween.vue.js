_$pi.define("app_c/demo/ui/show/animation/tween.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./tween.vue.tpl", "pi_gui/engine/animation_tools", "pi_utils/math/tween", "pi_gui/engine/tools", "pi_gui/engine/math_tools", "pi_gui/widget/direct", "pi_common/ui/main_root"], function (require, exports, module, direct_1, tween_vue_tpl_1, animation_tools_1, tween_1, tools_1, math_tools_1, direct_2, main_root_1) {
    "use strict";

    exports.initMeta = void 0;
    class TweenWidget {
        constructor() {
            this.playFg = false;
        }
        create() {
            math_tools_1.MathTools.register("quadInTween", tween_1.quadInTween);
            math_tools_1.MathTools.register("quadOutTween", tween_1.quadOutTween);
            math_tools_1.MathTools.register("cubicInTween", tween_1.cubicInTween);
        }
        click(num) {
            // js 控制动画
            if (num === 1) {
                let anim = sizeAnim();
                let dom = direct_2.getDomNode(direct_2.findElementByAttr(this, 'id', "box"));
                dom.style.addAnimation(anim);
                // CSS 控制动画
            } else {
                this.playFg = false;
                setTimeout(() => {
                    this.playFg = true;
                }, 100);
            }
        }
        close() {
            main_root_1.close(this);
        }
    }
    exports.default = TweenWidget;
    const sizeAnim = () => {
        const cmd = new tools_1.AnimationCmd("sizeAnim");
        cmd.iteration = 1;
        cmd.duration = 1000;
        cmd.fillMode = "forwards";
        cmd.timingFunction = tween_1.quadInTween; // 使用缓动函数
        const keyFrames = {
            name: "sizeAnim",
            attrs: [{
                key: animation_tools_1.FrameClassKeys.transform,
                data: [[0, tools_1.Tools.readTransform(tools_1.Tools.analy("transform", "translateX(0px)"))], [1, tools_1.Tools.readTransform(tools_1.Tools.analy("transform", "translateX(300px)"))]]
            }]
        };
        return animation_tools_1.AnimeTools.initRuntimeAnimation(cmd, keyFrames);
    };
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/animation/tween.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/animation/tween.vue.wcss",
            _$cssHash = 2792299705;
        TweenWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: tween_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TweenWidget, ["playFg"]);
});