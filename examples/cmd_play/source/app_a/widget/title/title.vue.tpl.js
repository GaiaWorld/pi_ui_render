_$pi.define("app_a/widget/title/title.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 3435969083, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "title_bar_bg"
                },
                class: 2737640275
            });
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 2456828211);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 117673441);
            direct_1.setText(t.n5, w.title);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 3435969083);
            if (dirty0 & 1) direct_1.setText(t.n5, w.title);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});