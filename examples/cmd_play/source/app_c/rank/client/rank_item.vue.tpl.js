_$pi.define("app_c/rank/client/rank_item.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s5() {
            let w = this.w;
            return w.i > 2 ? B5 : B6;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 1281916719, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "rank_bg4"
                },
                class: 2240836030
            });
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 1715669757);
            t.i5 = t.s5();
            t.n5 = direct_1.createIf(w, t.i5);
            t.n7 = direct_1.createDiv();
            direct_1.setClass(t.n7, 3683548995);
            t.n8 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "avator_bg"
                },
                class: 2584583663
            });
            t.n9 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "avator"
                },
                class: 959965456
            });
            t.n10 = direct_1.createDiv();
            direct_1.setClass(t.n10, 1505568156);
            t.n11 = direct_1.createSpan();
            direct_1.setClass(t.n11, 3182516286);
            direct_1.setText(t.n11, w.v.uid);
            t.n12 = direct_1.createDiv();
            direct_1.setClass(t.n12, 979950409);
            t.n13 = direct_1.createSpan();
            direct_1.setClass(t.n13, 3182516286);
            direct_1.setText(t.n13, w.v.num);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            t.n5.m(t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.mountChildWidget(t.n8, t.n7);
            direct_1.mountChildWidget(t.n9, t.n7);
            direct_1.append(t.n7, t.n2);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n10, t.n2);
            direct_1.append(t.n13, t.n12);
            direct_1.append(t.n12, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1281916719);
            t.n5 = direct_1.patchIf(w, t.n5, t.i5, t.i5 = t.s5(), t.n4);
            if (dirty0 & 2) direct_1.setText(t.n11, w.v.uid);
            if (dirty0 & 2) direct_1.setText(t.n13, w.v.num);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B6 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n6 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: 'rank_item_icon_' + (w.i + 1)
                },
                class: 2294339007
            });
            return this.n6;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n6);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n6, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.name = 'rank_item_icon_' + (w.i + 1);
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n6, _$wAttrs);
            return this.n6;
        }
    }
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 3648014830);
            direct_1.setText(t.n5, w.i + 1);
            return this.n5;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n5, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setText(t.n5, w.i + 1);
            return this.n5;
        }
    }
});