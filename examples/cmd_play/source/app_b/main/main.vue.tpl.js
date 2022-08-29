_$pi.define("app_b/main/main.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1, 1, 1], [0, 250], [1, 1], [0, 150], [1, 0, 0, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s13() {
            let w = this.w;
            return w.offline ? B13 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setAttr(t.n2, "isSendNextLayer", "true");
            direct_1.extendAttr(t.n2, w, 4246905833, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 7 /*top*/, w.barOffset + 10 + 'px');
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.addMoney();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 3263990429);
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 30 /*color*/, staticObj[0]);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 30);
            direct_1.setText(t.n4, "金币" + w.gold);
            t.n5 = direct_1.createDiv();
            direct_1.setEvent(t.n5, "mousemove", $event => {
                let r = w.mouseMove($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "over", $event => {
                let r = w.mouseOver($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "out", $event => {
                let r = w.mouseOut($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "enter", $event => {
                let r = w.mouseEnter($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "leave", $event => {
                let r = w.mouseLeave($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.mouseClick($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n5, 3186417848);
            t.n6 = direct_1.createSpan();
            direct_1.setEvent(t.n6, "over", $event => {
                let r = w.childMouseOver($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n6, "out", $event => {
                let r = w.childMouseOut($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n6, "enter", $event => {
                let r = w.childMouseEnter($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n6, "leave", $event => {
                let r = w.childMouseLeave($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n6, "鼠标事件测试22222");
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 6 /*position*/, 1);
            direct_1.setStyle(t.n7, 9 /*bottom*/, staticObj[1]);
            direct_1.setStyle(t.n7, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[2]);
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "底层"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px",
                    marginRight /*marginRight*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.nativeClick();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n9 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "音频播放"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    marginRight /*marginRight*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.audioClick();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n10 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "退出"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px",
                    marginRight /*marginRight*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.exitGame();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 6 /*position*/, 1);
            direct_1.setStyle(t.n11, 9 /*bottom*/, staticObj[3]);
            direct_1.setStyle(t.n11, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[2]);
            t.n12 = direct_1.createFor(w, w.btnList, B12);
            t.i13 = t.s13();
            t.n13 = direct_1.createIf(w, t.i13);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
            direct_1.destroyContext(t.n12);
            direct_1.destroyContext(t.n13);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.mountChildWidget(t.n8, t.n7);
            direct_1.mountChildWidget(t.n9, t.n7);
            direct_1.mountChildWidget(t.n10, t.n7);
            direct_1.append(t.n7, t.n2);
            t.n12.m(t.n11);
            direct_1.append(t.n11, t.n2);
            t.n13.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 4246905833);
            if (dirty0 & 1) direct_1.setStyle(t.n3, 7 /*top*/, w.barOffset + 10 + 'px');
            if (dirty0 & 2) direct_1.setText(t.n4, "金币" + w.gold);
            direct_1.patchFor(w, t.n12, w.btnList, B12);
            t.n13 = direct_1.patchIf(w, t.n13, t.i13, t.i13 = t.s13(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B13 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n13 = direct_1.createDiv();
            direct_1.setClass(t.n13, 3730912968);
            t.n14 = direct_1.createSpan();
            direct_1.setStyle(t.n14, 42 /*fontSize*/, 30);
            direct_1.setStyle(t.n14, 30 /*color*/, staticObj[4]);
            direct_1.setText(t.n14, "断网重连中...");
            return this.n13;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n14, t.n13);
            direct_1.insertBefore(t.n13, target, anchor);
        }
        p() {
            return this.n13;
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
            this.$i = i;
            t.n12 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    key: i,
                    text: v[0]
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "70px",
                    marginRight /*marginRight*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let i = t.$i;
                        let r = w.click(i);
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
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            this.$i = i;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = i;
                _$attrs.text = v[0];
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n12, _$wAttrs);
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
});