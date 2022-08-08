_$pi.define("app_c/fight/fight_main.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue", "./hp.vue", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, btn_vue_1, hp_vue_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.list.length === 0 ? B3 : B4;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 2356934053, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 3192896239);
            t.n7 = direct_1.createFor(w, w.list, B7);
            t.n11 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 171007438,
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
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n11);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n7.m(t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.mountChildWidget(t.n11, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 2356934053);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            direct_1.patchFor(w, t.n7, w.list, B7);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n7 = direct_1.createDiv();
            direct_1.setAttr(t.n7, "key", i);
            direct_1.setClass(t.n7, 3899233670);
            t.n8 = direct_1.createDiv();
            t.n9 = direct_1.createSpan();
            direct_1.setClass(t.n9, 149500231);
            direct_1.setText(t.n9, "我是:" + v);
            t.n10 = direct_1.createWidget(hp_vue_1.default, w, {
                attrs: {
                    id: v
                }
            });
            return this.n7;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n10);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n7);
            direct_1.mountChildWidget(t.n10, t.n7);
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n7, "key", i);
            direct_1.setText(t.n9, "我是:" + v);
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.id = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n10, _$wAttrs);
            return this.n7;
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
    class B4 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 3192896239);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 149500231);
            direct_1.setText(t.n5, w.over ? '游戏结束' : '游戏中。。。');
            return this.n4;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n5, t.n4);
            direct_1.insertBefore(t.n4, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 2) direct_1.setText(t.n5, w.over ? '游戏结束' : '游戏中。。。');
            return this.n4;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "开始游戏"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.start($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n3;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, target, anchor);
        }
        p() {
            return this.n3;
        }
    }
});