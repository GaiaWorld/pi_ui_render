_$pi.define("app_c/demo/demo_single.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [1, 1, 1, 1], [2, 0], [0.95, 0.88, 0.49, 1], [0, 10], [0, 50], [0.67, 0.98, 0.42, 1], [1, 0, 0, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "返回",
                    fontSize: "26"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "90px",
                    margin /*margin*/: "5px 10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.quit($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n4, 6 /*position*/, 0);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n4, 22 /*flexWrap*/, 1);
            t.n5 = direct_1.createFor(w, w.list, B5);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n7, 32 /*lineHeight*/, staticObj[5]);
            direct_1.setStyle(t.n7, 6 /*position*/, 0);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setStyle(t.n7, 61 /*display*/, 0);
            direct_1.setStyle(t.n7, 22 /*flexWrap*/, 1);
            t.n8 = direct_1.createSpan();
            direct_1.setStyle(t.n8, 42 /*fontSize*/, 26);
            direct_1.setStyle(t.n8, 30 /*color*/, staticObj[7]);
            direct_1.setText(t.n8, w.msg);
            t.n9 = direct_1.createSpan();
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
            t.n5.m(t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n8, t.n7);
            direct_1.append(t.n9, t.n7);
            direct_1.append(t.n7, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            direct_1.patchFor(w, t.n5, w.list, B5);
            if (dirty0 & 2) direct_1.setText(t.n8, w.msg);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n5 = direct_1.createDiv();
            direct_1.setAttr(t.n5, "key", i);
            direct_1.setStyle(t.n5, 17 /*marginTop*/, staticObj[4]);
            direct_1.setStyle(t.n5, 20 /*marginLeft*/, staticObj[4]);
            t.n6 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: v[0]
                },
                style: {
                    width /*width*/: "auto",
                    minWidth /*minWidth*/: "280px",
                    height /*height*/: "80px"
                },
                events: {
                    "pointerclick": $event => {
                        let i = t.$i;
                        let r = w.click(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n5;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n6);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n6, t.n5);
            direct_1.insertBefore(t.n5, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            direct_1.setAttr(t.n5, "key", i);
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.text = v[0];
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n6, _$wAttrs);
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