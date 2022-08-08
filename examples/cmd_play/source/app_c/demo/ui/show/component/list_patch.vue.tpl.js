_$pi.define("app_c/demo/ui/show/component/list_patch.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_common/ui/list.vue", "pi_common/ui/rubber.vue", "./list_patch.vue"], function (require, exports, module, direct_1, list_vue_1, rubber_vue_1, list_patch_vue_1) {
    "use strict";

    exports.BW11 = exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 50], [0, 100], [2, 0], [0, 40], [0, 0], [0, 30]];
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
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(rubber_vue_1.default, w, {
                attrs: {
                    ref: "rubber",
                    headHold: w.headHold,
                    footHold: w.footHold
                },
                style: {
                    backgroundColor /*backgroundColor*/: "rgb(255,0,0)",
                    height /*height*/: "100%",
                    width /*width*/: "100%"
                },
                slot: {
                    header: CreateSlot6,
                    footer: CreateSlot8,
                    default: CreateSlot3
                },
                scope: w
            });
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
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 3) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.headHold = w.headHold;
                if (dirty0 & 2) _$attrs.footHold = w.footHold;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 7) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n3, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class CreateSlot8 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n9 = direct_1.createSpan();
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[0]);
            direct_1.setText(t.n9, "下拉加载");
            return this.n9;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n9, target, anchor);
        }
        p() {
            return this.n9;
        }
    }
    class CreateSlot6 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[0]);
            direct_1.setText(t.n7, "上拉刷新...");
            return this.n7;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            return this.n7;
        }
    }
    class CreateSlot3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.ctx4 = [undefined, undefined];
            t.n4 = direct_1.createWidget(list_vue_1.default, w, {
                attrs: {
                    ref: "list",
                    data: w.arr,
                    finished: true
                },
                style: {
                    backgroundColor /*backgroundColor*/: "rgb(0,255,0)"
                },
                slot: {
                    default: CreateSlot4
                },
                sCtx: {
                    default: t.ctx4
                },
                scope: w,
                events: {
                    "ev-load": $event => {
                        let r = w.load();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n4;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 4) {
                const _$attrs = {};
                if (dirty0 & 4) _$attrs.data = w.arr;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 12) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n4, _$wAttrs);
            return this.n4;
        }
    }
    class CreateSlot4 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n5 = direct_1.createWidget(list_patch_vue_1.Item, w, {
                attrs: {
                    value: v.value,
                    state: v.state
                },
                events: {
                    "changeState": $event => {
                        let i = t.$i;
                        let r = w.changeState(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n5;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n5, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            var _$wAttrs = {};
            if (dirty0 & 8) {
                const _$attrs = {};
                if (dirty0 & 8) _$attrs.value = v.value;
                if (dirty0 & 8) _$attrs.state = v.state;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            return this.n5;
        }
        static getDirty({ v, i }) {
            return {
                v: v ? 3 : 0,
                i: i ? 4 : 0
            };
        }
        s({ v, i }) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
        }
    }
    class BW11 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, w.state ? '#f00' : '#00f');
            direct_1.extendAttr(t.n11, w, null, true);
            t.n12 = direct_1.createSpan();
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[3]);
            direct_1.setStyle(t.n12, 42 /*fontSize*/, 48);
            direct_1.setText(t.n12, w.value);
            t.n13 = direct_1.createSpan();
            direct_1.setStyle(t.n13, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n13, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n13, 6 /*position*/, 1);
            direct_1.setStyle(t.n13, 8 /*right*/, staticObj[5]);
            direct_1.setStyle(t.n13, 7 /*top*/, staticObj[6]);
            direct_1.setStyle(t.n13, 42 /*fontSize*/, 48);
            direct_1.setEvent(t.n13, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n13, "修改状态");
            return this.n11;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n13, t.n11);
            direct_1.insertBefore(t.n11, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setStyle(t.n11, 46 /*backgroundColor*/, w.state ? '#f00' : '#00f');
            direct_1.extendAttr(t.n11, w);
            if (dirty0 & 2) direct_1.setText(t.n12, w.value);
            return this.n11;
        }
    }
    exports.BW11 = BW11;
});