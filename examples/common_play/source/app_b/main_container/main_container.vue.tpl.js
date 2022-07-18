_$pi.define("app_b/main_container/main_container.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/menu/menu.vue"], function (require, exports, module, direct_1, menu_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s7() {
            let w = this.w;
            return w.showMenu ? B7 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 3400271547, true);
            t.n3 = direct_1.createDiv();
            direct_1.setAttr(t.n3, "id", "widget1");
            direct_1.setClass(t.n3, 2529755405);
            t.n4 = direct_1.createComponent(w, {
                attrs: {
                    is: w.widget1
                },
                class: 3400271547
            });
            t.n5 = direct_1.createDiv();
            direct_1.setAttr(t.n5, "id", "widget2");
            direct_1.setClass(t.n5, 988292225);
            t.n6 = direct_1.createComponent(w, {
                attrs: {
                    is: w.widget2
                },
                class: 3400271547
            });
            t.i7 = t.s7();
            t.n7 = direct_1.createIf(w, t.i7);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n6);
            direct_1.destroyContext(t.n7);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountComponent(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.mountComponent(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            t.n7.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 3400271547);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.is = w.widget1;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            t.n4 = direct_1.patchAttrsForComponent(t.n4, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 2) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.is = w.widget2;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            t.n6 = direct_1.patchAttrsForComponent(t.n6, _$wAttrs);
            t.n7 = direct_1.patchIf(w, t.n7, t.i7, t.i7 = t.s7(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n7 = direct_1.createDiv();
            direct_1.setClass(t.n7, 1981362054);
            t.n8 = direct_1.createDiv();
            direct_1.setClass(t.n8, 1544334741);
            t.n9 = direct_1.createDiv();
            direct_1.setClass(t.n9, 2690618721);
            t.n10 = direct_1.createFor(w, w.bottomMenu, B10);
            return this.n7;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n10);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n8, t.n7);
            t.n10.m(t.n9);
            direct_1.append(t.n9, t.n7);
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.patchFor(w, t.n10, w.bottomMenu, B10);
            return this.n7;
        }
    }
    class B10 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n10 = direct_1.createWidget(menu_vue_1.default, w, {
                attrs: {
                    key: i,
                    image: v.image,
                    showAnim: v.showAnim,
                    canUse: v.canUse,
                    selected: v.selected
                },
                events: {
                    "pointerclick": $event => {
                        let i = t.$i;
                        let r = w.openinterface(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    },
                    "pointerdown": $event => {
                        let i = t.$i;
                        let r = w.onDown(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    },
                    "pointermove": $event => {
                        let r = w.sliding($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    },
                    "pointerup": $event => {
                        let r = w.upDefault($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n10;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n10);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n10, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = i;
                _$attrs.image = v.image;
                _$attrs.showAnim = v.showAnim;
                _$attrs.canUse = v.canUse;
                _$attrs.selected = v.selected;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n10, _$wAttrs);
            return this.n10;
        }
        s(v, i) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
        }
        getKey(v, i) {
            return i;
        }
    }
});