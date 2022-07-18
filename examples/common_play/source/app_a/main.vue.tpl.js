_$pi.define("app_a/main.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue", "app_b/vue_widget/window_bg.vue", "app_b/vue_widget/goback.vue", "app_b/vue_widget/title.vue", "app/vue_widget/red_point.vue", "app/vue_widget/pic_menu.vue", "app_c/day7_activity/client/box.vue", "app_c/day7_activity/client/day7_list.vue", "app/vue_widget/btn.vue", "./main.vue"], function (require, exports, module, direct_1, imgmap_vue_1, window_bg_vue_1, goback_vue_1, title_vue_1, red_point_vue_1, pic_menu_vue_1, box_vue_1, day7_list_vue_1, btn_vue_1, main_vue_1) {
    "use strict";

    exports.BW20 = exports.BW2 = void 0;
    const staticObj = [[2, 0], [1, 1], [0, 208], [0, 86], [null, [0, 0], [0, 1], [0, 0], [0, 1]]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s11() {
            let w = this.w;
            return w.smallTypes.length > 1 ? B11 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 45295019, true);
            t.n3 = direct_1.createWidget(window_bg_vue_1.default, w, {
                attrs: {
                    url: "cz_kaoshangling1_bg"
                }
            });
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "chongzuan1_fangding"
                },
                class: 3225787308
            });
            t.n5 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "cz_kaoshangling1_dibu_kuang"
                },
                class: 129895777
            });
            t.n6 = direct_1.createWidget(title_vue_1.default, w, {
                attrs: {
                    onlyShowCoin: true,
                    coin: [101, 102]
                },
                style: {
                    top /*top*/: "20px"
                }
            });
            t.n7 = direct_1.createDiv();
            direct_1.setClass(t.n7, 1194394860);
            t.n8 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "chongzuan2_guanggaotu"
                },
                class: 45295019
            });
            t.n9 = direct_1.createWidget(box_vue_1.default, w, {
                attrs: {
                    value: w.value
                }
            });
            t.n10 = direct_1.createWidget(day7_list_vue_1.default, w, {
                attrs: {
                    bigType: w.selectBigTypeId,
                    smallType: w.selectSmallTypeId
                },
                class: 2839023949,
                events: {
                    "ev-update-value": $event => {
                        let r = w.updateValue($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.i11 = t.s11();
            t.n11 = direct_1.createIf(w, t.i11);
            t.n13 = direct_1.createDiv();
            direct_1.setAttr(t.n13, "scroll_type", "none");
            direct_1.setAttr(t.n13, "scroll_path", w.is_scroll ? 'x' : 'none');
            direct_1.setStyle(t.n13, 0 /*width*/, w.is_scroll ? '100%' : 'auto');
            direct_1.setClass(t.n13, 2427514110);
            t.n14 = direct_1.createDiv();
            direct_1.setStyle(t.n14, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n14, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n14, 6 /*position*/, 0);
            t.n15 = direct_1.createFor(w, w.bgTypes, B15);
            t.n18 = direct_1.createWidget(goback_vue_1.default, w, {
                attrs: {
                    width: "289px",
                    onlyLeft: true
                },
                events: {
                    "ev-goback": $event => {
                        let r = w.handleGoBack($event);
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
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n6);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
            direct_1.destroyContext(t.n11);
            direct_1.destroyContext(t.n15);
            direct_1.destroyContext(t.n18);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
            direct_1.mountChildWidget(t.n6, t.n2);
            direct_1.mountChildWidget(t.n8, t.n7);
            direct_1.mountChildWidget(t.n9, t.n7);
            direct_1.mountChildWidget(t.n10, t.n7);
            t.n11.m(t.n7);
            direct_1.append(t.n7, t.n2);
            t.n15.m(t.n14);
            direct_1.append(t.n14, t.n13);
            direct_1.append(t.n13, t.n2);
            direct_1.mountChildWidget(t.n18, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 45295019);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.value = w.value;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n9, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 6) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.bigType = w.selectBigTypeId;
                if (dirty0 & 4) _$attrs.smallType = w.selectSmallTypeId;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n10, _$wAttrs);
            t.n11 = direct_1.patchIf(w, t.n11, t.i11, t.i11 = t.s11(), t.n7);
            if (dirty0 & 16) direct_1.setAttr(t.n13, "scroll_path", w.is_scroll ? 'x' : 'none');
            if (dirty0 & 16) direct_1.setStyle(t.n13, 0 /*width*/, w.is_scroll ? '100%' : 'auto');
            direct_1.patchFor(w, t.n15, w.bgTypes, B15);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B15 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s17() {
            let w = this.w;
            let [v, i] = this._$ctx;
            return w.redCondition[v.id] ? B17 : null;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n15 = direct_1.createDiv();
            direct_1.setAttr(t.n15, "key", v.id);
            direct_1.setStyle(t.n15, 20 /*marginLeft*/, (i == 0 ? 200 : 26) + 'px');
            direct_1.setClass(t.n15, 187737861);
            t.n16 = direct_1.createWidget(pic_menu_vue_1.default, w, {
                attrs: {
                    text: v.name,
                    select: v.id === w.selectBigTypeId
                },
                events: {
                    "pointerclick": $event => {
                        let i = t.$i;
                        let r = w.changeBigType(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.i17 = t.s17();
            t.n17 = direct_1.createIf(w, t.i17);
            return this.n15;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n16);
            direct_1.destroyContext(t.n17);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n16, t.n15);
            t.n17.m(t.n15);
            direct_1.insertBefore(t.n15, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            direct_1.setAttr(t.n15, "key", v.id);
            direct_1.setStyle(t.n15, 20 /*marginLeft*/, (i == 0 ? 200 : 26) + 'px');
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.text = v.name;
                _$attrs.select = v.id === w.selectBigTypeId;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n16, _$wAttrs);
            t.n17 = direct_1.patchIf(w, t.n17, t.i17, t.i17 = t.s17(), t.n15);
            return this.n15;
        }
        s(v, i) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
        }
        getKey(v, i) {
            return v.id;
        }
    }
    class B17 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n17 = direct_1.createWidget(red_point_vue_1.default, w, {
                attrs: {
                    condition: w.redCondition[v.id]
                },
                class: 4186260321
            });
            return this.n17;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n17);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n17, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 64) {
                const _$attrs = {};
                if (dirty0 & 64) _$attrs.condition = w.redCondition[v.id];
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n17, _$wAttrs);
            return this.n17;
        }
    }
    class B11 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n11 = direct_1.createDiv();
            direct_1.setClass(t.n11, 2243422399);
            t.n12 = direct_1.createFor(w, w.smallTypes, B12);
            return this.n11;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n12);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n12.m(t.n11);
            direct_1.insertBefore(t.n11, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.patchFor(w, t.n12, w.smallTypes, B12);
            return this.n11;
        }
    }
    class B12 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            this.$v = v;
            t.n12 = direct_1.createWidget(main_vue_1.SmallTypeBtn, w, {
                attrs: {
                    key: v,
                    id: v,
                    select: w.selectSmallTypeId === v
                },
                events: {
                    "pointerclick": $event => {
                        let v = t.$v;
                        let r = w.handleChangeSmallType(v);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n12;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n12);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n12, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$v = v;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = v;
                _$attrs.id = v;
                _$attrs.select = w.selectSmallTypeId === v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n12, _$wAttrs);
            return this.n12;
        }
        s(v) {
            let w = this.w;
            this._$ctx[0] = v;
        }
        getKey(v) {
            return v;
        }
    }
    class BW20 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n20 = direct_1.createDiv();
            direct_1.setStyle(t.n20, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n20, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n20, 6 /*position*/, 0);
            direct_1.setStyle(t.n20, 16 /*margin*/, staticObj[4]);
            direct_1.extendAttr(t.n20, w, null, true);
            t.n21 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    bg: w.bg,
                    text: w.name,
                    textClass: w.textClass,
                    textTop: -2,
                    effect: true
                },
                class: 45295019
            });
            return this.n20;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n21);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n21, t.n20);
            direct_1.insertBefore(t.n20, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n20, w);
            var _$wAttrs = {};
            if (dirty0 & 7) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.bg = w.bg;
                if (dirty0 & 2) _$attrs.text = w.name;
                if (dirty0 & 4) _$attrs.textClass = w.textClass;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n21, _$wAttrs);
            return this.n20;
        }
    }
    exports.BW20 = BW20;
});