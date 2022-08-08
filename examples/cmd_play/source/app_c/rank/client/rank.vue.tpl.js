_$pi.define("app_c/rank/client/rank.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/title/title.vue", "./rank_item.vue", "pi_gui/ui/imgmap.vue", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, title_vue_1, rank_item_vue_1, imgmap_vue_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s20() {
            let w = this.w;
            return w.selfRank > 2 ? B20 : w.selfRank === 0 ? B21 : B22;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 2455624308, true);
            t.n3 = direct_1.createDiv();
            direct_1.setAttr(t.n3, "scroll_path", "y");
            direct_1.setAttr(t.n3, "layout", "scroll");
            direct_1.setAttr(t.n3, "scroll_type", "none");
            direct_1.setAttr(t.n3, "id", "rankList");
            direct_1.setClass(t.n3, 3602274759);
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 2284307285);
            t.n5 = direct_1.createFor(w, w.rankList, B5);
            t.n6 = direct_1.createDiv();
            direct_1.setAttr(t.n6, "e-show", "header");
            direct_1.setClass(t.n6, 2246059135);
            t.n7 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "head_bg"
                },
                class: 2979189095
            });
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "更新数据"
                },
                style: {
                    width /*width*/: "220px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.update();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n9 = direct_1.createWidget(title_vue_1.default, w, {
                attrs: {
                    title: '排行榜'
                },
                style: {
                    marginTop /*marginTop*/: "30px"
                }
            });
            t.n10 = direct_1.createDiv();
            direct_1.setClass(t.n10, 1448626917);
            t.n11 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "rank_title_bg"
                },
                class: 423940068
            });
            t.n12 = direct_1.createFor(w, w.arr, B12);
            t.n14 = direct_1.createDiv();
            direct_1.setClass(t.n14, 2593645102);
            t.n15 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "rank_bg0"
                },
                class: 423940068
            });
            t.n16 = direct_1.createSpan();
            direct_1.setClass(t.n16, 946655986);
            direct_1.setText(t.n16, "我的排名");
            t.n17 = direct_1.createDiv();
            direct_1.setClass(t.n17, 1039915825);
            t.n18 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "rank_bg5"
                },
                class: 423940068
            });
            t.n19 = direct_1.createDiv();
            direct_1.setClass(t.n19, 2054471870);
            t.i20 = t.s20();
            t.n20 = direct_1.createIf(w, t.i20);
            t.n23 = direct_1.createDiv();
            direct_1.setClass(t.n23, 2564619914);
            t.n24 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "avator_bg"
                },
                class: 2233471914
            });
            t.n25 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "avator"
                },
                class: 1036042841
            });
            t.n26 = direct_1.createDiv();
            direct_1.setClass(t.n26, 2794611510);
            t.n27 = direct_1.createSpan();
            direct_1.setClass(t.n27, 4048155554);
            direct_1.setText(t.n27, w.role.rid);
            t.n28 = direct_1.createDiv();
            direct_1.setClass(t.n28, 3807900888);
            t.n29 = direct_1.createSpan();
            direct_1.setClass(t.n29, 4048155554);
            direct_1.setText(t.n29, w.role.heart);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n11);
            direct_1.destroyContext(t.n12);
            direct_1.destroyContext(t.n15);
            direct_1.destroyContext(t.n18);
            direct_1.destroyContext(t.n20);
            direct_1.destroyContext(t.n24);
            direct_1.destroyContext(t.n25);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n5.m(t.n4);
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.mountChildWidget(t.n7, t.n6);
            direct_1.mountChildWidget(t.n8, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.mountChildWidget(t.n9, t.n2);
            direct_1.mountChildWidget(t.n11, t.n10);
            t.n12.m(t.n10);
            direct_1.append(t.n10, t.n2);
            direct_1.mountChildWidget(t.n15, t.n14);
            direct_1.append(t.n16, t.n14);
            direct_1.append(t.n14, t.n2);
            direct_1.mountChildWidget(t.n18, t.n17);
            t.n20.m(t.n19);
            direct_1.append(t.n19, t.n17);
            direct_1.mountChildWidget(t.n24, t.n23);
            direct_1.mountChildWidget(t.n25, t.n23);
            direct_1.append(t.n23, t.n17);
            direct_1.append(t.n27, t.n26);
            direct_1.append(t.n26, t.n17);
            direct_1.append(t.n29, t.n28);
            direct_1.append(t.n28, t.n17);
            direct_1.append(t.n17, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 2455624308);
            direct_1.patchFor(w, t.n5, w.rankList, B5);
            direct_1.patchFor(w, t.n12, w.arr, B12);
            t.n20 = direct_1.patchIf(w, t.n20, t.i20, t.i20 = t.s20(), t.n19);
            if (dirty0 & 8) direct_1.setText(t.n27, w.role.rid);
            if (dirty0 & 8) direct_1.setText(t.n29, w.role.heart);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B22 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n22 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: 'rank_icon_' + w.selfRank
                },
                class: 3534815721
            });
            return this.n22;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n22);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n22, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 4) {
                const _$attrs = {};
                if (dirty0 & 4) _$attrs.name = 'rank_icon_' + w.selfRank;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n22, _$wAttrs);
            return this.n22;
        }
    }
    class B21 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n21 = direct_1.createSpan();
            direct_1.setClass(t.n21, 3015397928);
            direct_1.setText(t.n21, "未上榜");
            return this.n21;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n21, target, anchor);
        }
        p() {
            return this.n21;
        }
    }
    class B20 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n20 = direct_1.createSpan();
            direct_1.setClass(t.n20, 3015397928);
            direct_1.setText(t.n20, w.selfRank);
            return this.n20;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n20, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 4) direct_1.setText(t.n20, w.selfRank);
            return this.n20;
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
            let [v, i] = t._$ctx;
            t.n12 = direct_1.createDiv();
            direct_1.setAttr(t.n12, "key", i);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n12, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n12, 27 /*alignContent*/, 2);
            direct_1.setStyle(t.n12, 0 /*width*/, v.width + 'px');
            t.n13 = direct_1.createSpan();
            direct_1.setClass(t.n13, 614243436);
            direct_1.setText(t.n13, v.title);
            return this.n12;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n13, t.n12);
            direct_1.insertBefore(t.n12, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n12, "key", i);
            direct_1.setStyle(t.n12, 0 /*width*/, v.width + 'px');
            direct_1.setText(t.n13, v.title);
            return this.n12;
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
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n5 = direct_1.createWidget(rank_item_vue_1.default, w, {
                attrs: {
                    key: i,
                    i: i,
                    v: v
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
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = i;
                _$attrs.i = i;
                _$attrs.v = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            return this.n5;
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