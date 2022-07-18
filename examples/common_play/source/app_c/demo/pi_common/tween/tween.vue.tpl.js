_$pi.define("app_c/demo/pi_common/tween/tween.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1, 1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 3035067594, true);
            t.n3 = direct_1.createDiv();
            direct_1.setAttr(t.n3, "id", "test");
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.click($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 4059303539);
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "login_btn"
                },
                class: 371067036
            });
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 30);
            direct_1.setStyle(t.n5, 30 /*color*/, staticObj[0]);
            direct_1.setText(t.n5, "点击播动画");
            t.n6 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 3490352516,
                events: {
                    "pointerclick": $event => {
                        let r = w.closePage($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n6);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, t.n3);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.mountChildWidget(t.n6, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w, 3035067594);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});