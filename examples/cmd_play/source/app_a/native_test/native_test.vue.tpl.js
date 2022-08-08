_$pi.define("app_a/native_test/native_test.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [1, 1, 1, 1], [0, 100], [0, 0], [0, 200], [2, 0], [0.95, 0.88, 0.49, 1], [0, 300]];
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
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 7 /*top*/, staticObj[3]);
            direct_1.setStyle(t.n3, 22 /*flexWrap*/, 1);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[4]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            t.n5 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "退出",
                    fontSize: "26"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "90px",
                    position /*position*/: "absolute",
                    left /*left*/: "10px",
                    top /*top*/: "5px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.quit($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[5]);
            direct_1.setStyle(t.n6, 6 /*position*/, 1);
            direct_1.setStyle(t.n6, 7 /*top*/, staticObj[2]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setStyle(t.n6, 22 /*flexWrap*/, 1);
            t.n7 = direct_1.createFor(w, w.cfglist, B7);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n7);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n5, t.n4);
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            t.n7.m(t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            direct_1.patchFor(w, t.n7, w.cfglist, B7);
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
            this.$i = i;
            t.n7 = direct_1.createDiv();
            direct_1.setAttr(t.n7, "key", i);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[2]);
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: v[0]
                },
                style: {
                    width /*width*/: "280px",
                    height /*height*/: "80px",
                    position /*position*/: "absolute",
                    left /*left*/: "10px",
                    top /*top*/: "10px"
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
            return this.n7;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n8);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n8, t.n7);
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            direct_1.setAttr(t.n7, "key", i);
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.text = v[0];
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n8, _$wAttrs);
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
});