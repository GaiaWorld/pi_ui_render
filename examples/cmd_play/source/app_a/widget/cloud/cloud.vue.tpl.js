_$pi.define("app_a/widget/cloud/cloud.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setAttr(t.n2, "id", "cloudBox");
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 64 /*zIndex*/, 3);
            direct_1.setStyle(t.n2, 63 /*pointerEvents*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "cloud_left"
                },
                style: {
                    width /*width*/: "100%",
                    transform /*transform*/: "translateX(-100%)",
                    position /*position*/: "absolute"
                }
            });
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "cloud_right"
                },
                style: {
                    width /*width*/: "100%",
                    transform /*transform*/: "translateX(100%)",
                    position /*position*/: "absolute"
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