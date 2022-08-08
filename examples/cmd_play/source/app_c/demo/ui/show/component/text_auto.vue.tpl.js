_$pi.define("app_c/demo/ui/show/component/text_auto.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_common/ui/text_auto.vue"], function (require, exports, module, direct_1, text_auto_vue_1) {
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
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(text_auto_vue_1.default, w, {
                style: {
                    backgroundColor /*backgroundColor*/: "#ff0000"
                },
                slot: {
                    default: CreateSlot3
                },
                scope: w
            });
            t.n5 = direct_1.createWidget(text_auto_vue_1.default, w, {
                style: {
                    width /*width*/: "70%",
                    height /*height*/: "30px",
                    backgroundColor /*backgroundColor*/: "#ff00ff"
                },
                slot: {
                    default: CreateSlot5
                },
                scope: w
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
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
    class CreateSlot5 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n6 = direct_1.createSpan();
            direct_1.setText(t.n6, "document节点和element节点的属性");
            return this.n6;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n6, target, anchor);
        }
        p() {
            return this.n6;
        }
    }
    class CreateSlot3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n4 = direct_1.createSpan();
            direct_1.setText(t.n4, "document节点和element节点的属性.");
            return this.n4;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n4, target, anchor);
        }
        p() {
            return this.n4;
        }
    }
});