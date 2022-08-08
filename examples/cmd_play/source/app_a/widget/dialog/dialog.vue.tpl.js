_$pi.define("app_a/widget/dialog/dialog.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s10() {
            let w = this.w;
            return w.showCloseTip ? B10 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setAttr(t.n2, "keeptransform", w.keep || 1);
            direct_1.setEvent(t.n2, "pointerclick", $event => {
                let r = w.maskClick($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n2, w, 110238961, true);
            t.n3 = direct_1.createDiv();
            direct_1.setClass(t.n3, 2250910352);
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_bg"
                },
                class: 2513529208
            });
            t.n5 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_title_bg"
                },
                class: 3214186052
            });
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 1977893662);
            t.n7 = direct_1.createSpan();
            direct_1.setClass(t.n7, 721359539);
            direct_1.setText(t.n7, w.title);
            t.n8 = direct_1.createComponent(w, {
                attrs: {
                    is: w.widget
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.empty($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n9 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 2002378296,
                events: {
                    "pointerclick": $event => {
                        let r = w.handleClose($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.i10 = t.s10();
            t.n10 = direct_1.createIf(w, t.i10);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, t.n3);
            direct_1.mountChildWidget(t.n5, t.n3);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n3);
            direct_1.mountComponent(t.n8, t.n3);
            direct_1.mountChildWidget(t.n9, t.n3);
            direct_1.append(t.n3, t.n2);
            t.n10.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setAttr(t.n2, "keeptransform", w.keep || 1);
            direct_1.extendAttr(t.n2, w, 110238961);
            if (dirty0 & 2) direct_1.setText(t.n7, w.title);
            var _$wAttrs = {};
            if (dirty0 & 4) {
                const _$attrs = {};
                if (dirty0 & 4) _$attrs.is = w.widget;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            t.n8 = direct_1.patchAttrsForComponent(t.n8, _$wAttrs);
            t.n10 = direct_1.patchIf(w, t.n10, t.i10, t.i10 = t.s10(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B10 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n10 = direct_1.createDiv();
            direct_1.setClass(t.n10, 2001229577);
            t.n11 = direct_1.createSpan();
            direct_1.setStyle(t.n11, 30 /*color*/, w.textColor || '#7e7e7e');
            direct_1.setClass(t.n11, 1602369512);
            direct_1.setText(t.n11, "点击屏幕任意位置退出");
            return this.n10;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n11, t.n10);
            direct_1.insertBefore(t.n10, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 16) direct_1.setStyle(t.n11, 30 /*color*/, w.textColor || '#7e7e7e');
            return this.n10;
        }
    }
});