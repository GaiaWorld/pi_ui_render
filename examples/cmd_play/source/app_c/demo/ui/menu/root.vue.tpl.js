_$pi.define("app_c/demo/ui/menu/root.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./treemenu.vue"], function (require, exports, module, direct_1, treemenu_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [1, 1, 1, 1], [0, 180], [0, 90], [null, [0, 5], [0, 10], [0, 5], [0, 10]], [0.95, 0, 0.03, 1], [0, 10], [0.14, 0.15, 0.16, 1], [0, 500], [0, 0], [0, 200], [null, [0, 0], [0, 0], [0, 0], [0, 2]], [0.85, 0.85, 0.85, 1], [0, 40], [1, 0, 0, 1], [2, 0]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s8() {
            let w = this.w;
            return w.show ? B8 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 16 /*margin*/, staticObj[4]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setStyle(t.n3, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n3, 29 /*justifyContent*/, 2);
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.quit($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 30);
            direct_1.setText(t.n4, "返回");
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[7]);
            direct_1.setStyle(t.n5, 23 /*flexGrow*/, 1);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n6, 23 /*flexGrow*/, 1);
            t.n7 = direct_1.createComponent(w, {
                attrs: {
                    is: w.widget
                }
            });
            t.i8 = t.s8();
            t.n8 = direct_1.createIf(w, t.i8);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n8);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.mountComponent(t.n7, t.n6);
            direct_1.append(t.n6, t.n5);
            t.n8.m(t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.is = w.widget;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            t.n7 = direct_1.patchAttrsForComponent(t.n7, _$wAttrs);
            t.n8 = direct_1.patchIf(w, t.n8, t.i8, t.i8 = t.s8(), t.n5);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B8 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 8 /*right*/, staticObj[9]);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n8, 21 /*flexDirection*/, 1);
            direct_1.setStyle(t.n8, 51 /*borderWidth*/, staticObj[11]);
            direct_1.setStyle(t.n8, 52 /*borderColor*/, staticObj[12]);
            direct_1.setStyle(t.n8, null /*borderStyle*/, "solid");
            direct_1.setStyle(t.n8, 65 /*opacity*/, 0.7);
            t.n9 = direct_1.createDiv();
            direct_1.setAttr(t.n9, "on-tap", "showMenu");
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[13]);
            direct_1.setStyle(t.n9, 28 /*alignItems*/, 2);
            t.n10 = direct_1.createSpan();
            direct_1.setStyle(t.n10, 30 /*color*/, staticObj[14]);
            direct_1.setStyle(t.n10, 42 /*fontSize*/, 30);
            direct_1.setText(t.n10, "menu");
            t.n11 = direct_1.createDiv();
            direct_1.setAttr(t.n11, "scroll_path", "y");
            direct_1.setAttr(t.n11, "scroll_type", "none");
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n11, 23 /*flexGrow*/, 1);
            direct_1.setStyle(t.n11, 66 /*overflow*/, true);
            direct_1.setStyle(t.n11, 61 /*display*/, w.tree.show ? 'block' : 'none');
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[15]);
            direct_1.setStyle(t.n12, 21 /*flexDirection*/, 0);
            direct_1.setStyle(t.n12, 22 /*flexWrap*/, 1);
            t.n13 = direct_1.createWidget(treemenu_vue_1.default, w, {
                attrs: {
                    tree: w.tree,
                    curIndex: 0,
                    btnWidget: w.btnWidget
                },
                style: {
                    width /*width*/: "100%"
                },
                events: {
                    "ev-open": $event => {
                        let r = w.open($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n8;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n13);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n10, t.n9);
            direct_1.append(t.n9, t.n8);
            direct_1.mountChildWidget(t.n13, t.n12);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n11, t.n8);
            direct_1.insertBefore(t.n8, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 4) direct_1.setStyle(t.n11, 61 /*display*/, w.tree.show ? 'block' : 'none');
            var _$wAttrs = {};
            if (dirty0 & 12) {
                const _$attrs = {};
                if (dirty0 & 4) _$attrs.tree = w.tree;
                if (dirty0 & 8) _$attrs.btnWidget = w.btnWidget;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n13, _$wAttrs);
            return this.n8;
        }
    }
});