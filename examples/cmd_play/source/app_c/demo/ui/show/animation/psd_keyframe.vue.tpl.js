_$pi.define("app_c/demo/ui/show/animation/psd_keyframe.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[{
        duration: 2000,
        timingFunction: "step",
        delayTime: 0,
        iteration: 1,
        direction: "direction",
        fillMode: "forwards",
        name: "get_money"
    }]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "get_money"
                },
                style: {
                    width /*width*/: "500px",
                    height /*height*/: "800px",
                    position /*position*/: "relative"
                }
            });
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "get_money"
                },
                style: {
                    width /*width*/: "500px",
                    height /*height*/: "800px",
                    position /*position*/: "relative",
                    animation /*animation*/: direct_1.createRunTimeAnimation(staticObj[0], w)
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n4);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});