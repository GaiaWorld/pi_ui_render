_$pi.define("app_c/demo/ui/show/test/vue.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./vue.vue"], function (require, exports, module, direct_1, vue_vue_1) {
    "use strict";

    exports.BW61 = exports.BW59 = exports.BW54 = exports.BW50 = exports.BW47 = exports.BW43 = exports.BW38 = exports.BW34 = exports.BW31 = exports.BW27 = exports.BW25 = exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 100], [0, 200], [1, 1, 0.2, 1], [{
        duration: 2000,
        timingFunction: "linear",
        delayTime: 0,
        iteration: 5,
        direction: "direction",
        fillMode: "none",
        name: "testAnim"
    }], [0, 30], [0, 1, 0, 1], [0, 50], [1, 0, 0, 1], [0, 0, 1, 1], [0, 500], [1, 1, 1, 1], [[0, 10], null, null, null, null], [0, 10]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n3, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, w.color);
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.close();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n4 = direct_1.createDiv();
            direct_1.setEvent(t.n4, "pointerclick", $event => {
                let r = w.changeClazzName();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n4, [2869555723, direct_1.classHash(w.className), 389030959, direct_1.classHash(w.clazzName, w)]);
            t.n5 = direct_1.createDiv();
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.changeClazzName1();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n5, [2869555723, direct_1.classHash(w.className1), 389030959, direct_1.classHash(w.clazzName1, w)]);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n6, 70 /*animation*/, direct_1.createRunTimeAnimation(staticObj[4], w));
            direct_1.setEvent(t.n6, "animationstart", $event => {
                let r = w.animationStart($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n6, "animationiteration", $event => {
                let r = w.animationIter($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n6, "animationend", $event => {
                let r = w.animationEnd($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n7 = direct_1.createWidget(vue_vue_1.TestSingleInner, w, {
                attrs: {
                    clazzName: w.clazzName,
                    className: w.className,
                    color: w.color
                },
                style: {
                    width /*width*/: "150px"
                },
                class: 3412889436
            });
            t.n8 = direct_1.createWidget(vue_vue_1.TestIf, w, null);
            t.n9 = direct_1.createWidget(vue_vue_1.TestShow, w, null);
            t.n10 = direct_1.createWidget(vue_vue_1.TestFor, w, null);
            t.n11 = direct_1.createWidget(vue_vue_1.TestForWithKey, w, null);
            t.n12 = direct_1.createWidget(vue_vue_1.TestModel, w, {
                model: w.inputValue
            });
            direct_1.modelBind(t.n12, v => {
                w.inputValue = v;
            });
            t.n13 = direct_1.createWidget(vue_vue_1.TestModel, w, {
                model: w.inputValue
            });
            direct_1.modelBind(t.n13, v => {
                w.inputValue = v;
            });
            t.n14 = direct_1.createSpan();
            direct_1.setStyle(t.n14, 1 /*height*/, staticObj[5]);
            direct_1.setStyle(t.n14, 42 /*fontSize*/, 30);
            direct_1.setStyle(t.n14, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setEvent(t.n14, "pointerclick", $event => {
                let r = w.changeSlot();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n14, "修改slot");
            t.ctx15 = [undefined, undefined];
            t.n15 = direct_1.createWidget(vue_vue_1.TestSlot, w, {
                slot: {
                    default: CreateSlot15
                },
                sCtx: {
                    default: t.ctx15
                },
                scope: w
            });
            t.n17 = direct_1.createWidget(vue_vue_1.TestSlot, w, null);
            t.ctx18 = [undefined, undefined];
            t.n18 = direct_1.createWidget(vue_vue_1.TestSlotWithDefault, w, {
                slot: {
                    default: CreateSlot18
                },
                sCtx: {
                    default: t.ctx18
                },
                scope: w
            });
            t.n20 = direct_1.createWidget(vue_vue_1.TestSlotWithDefault, w, null);
            t.ctx21 = [undefined, undefined];
            t.n21 = direct_1.createWidget(vue_vue_1.TestSlotWithDefaultMult, w, {
                slot: {
                    default: CreateSlot21
                },
                sCtx: {
                    default: t.ctx21
                },
                scope: w
            });
            t.n23 = direct_1.createWidget(vue_vue_1.TestSlotWithDefaultMult, w, null);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
            direct_1.destroyContext(t.n11);
            direct_1.destroyContext(t.n12);
            direct_1.destroyContext(t.n13);
            direct_1.destroyContext(t.n15);
            direct_1.destroyContext(t.n17);
            direct_1.destroyContext(t.n18);
            direct_1.destroyContext(t.n20);
            direct_1.destroyContext(t.n21);
            direct_1.destroyContext(t.n23);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n6, t.n2);
            direct_1.mountChildWidget(t.n7, t.n2);
            direct_1.mountChildWidget(t.n8, t.n2);
            direct_1.mountChildWidget(t.n9, t.n2);
            direct_1.mountChildWidget(t.n10, t.n2);
            direct_1.mountChildWidget(t.n11, t.n2);
            direct_1.mountChildWidget(t.n12, t.n2);
            direct_1.mountChildWidget(t.n13, t.n2);
            direct_1.append(t.n14, t.n2);
            direct_1.mountChildWidget(t.n15, t.n2);
            direct_1.mountChildWidget(t.n17, t.n2);
            direct_1.mountChildWidget(t.n18, t.n2);
            direct_1.mountChildWidget(t.n20, t.n2);
            direct_1.mountChildWidget(t.n21, t.n2);
            direct_1.mountChildWidget(t.n23, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            if (dirty0 & 1) direct_1.setStyle(t.n3, 46 /*backgroundColor*/, w.color);
            if (dirty0 & 2) direct_1.setClass(t.n4, [2869555723, direct_1.classHash(w.className), 389030959, direct_1.classHash(w.clazzName, w)]);
            if (dirty0 & 8) direct_1.setClass(t.n5, [2869555723, direct_1.classHash(w.className1), 389030959, direct_1.classHash(w.clazzName1, w)]);
            var _$wAttrs = {};
            if (dirty0 & 3) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.clazzName = w.clazzName;
                if (dirty0 & 1) _$attrs.color = w.color;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n7, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 32) _$wAttrs.model = w.inputValue;
            ;
            direct_1.patchAttrsForWidget(t.n12, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 32) _$wAttrs.model = w.inputValue;
            ;
            direct_1.patchAttrsForWidget(t.n13, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 448) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n15, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 1600) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n18, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 6208) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n21, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class CreateSlot21 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            t.n22 = direct_1.createSpan();
            direct_1.setText(t.n22, w.slot + slotName + other);
            return this.n22;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n22, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            let { dirty0 } = w._$info;
            if (dirty0 & 6208) direct_1.setText(t.n22, w.slot + slotName + other);
            return this.n22;
        }
        static getDirty({ slotName, other }) {
            return {
                slotName: slotName ? 11 : 0,
                other: other ? 12 : 0
            };
        }
        s({ slotName, other }) {
            let w = this.w;
            this._$ctx[0] = slotName;
            this._$ctx[1] = other;
        }
    }
    class CreateSlot18 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            t.n19 = direct_1.createSpan();
            direct_1.setText(t.n19, w.slot + slotName + other);
            return this.n19;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n19, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            let { dirty0 } = w._$info;
            if (dirty0 & 1600) direct_1.setText(t.n19, w.slot + slotName + other);
            return this.n19;
        }
        static getDirty({ slotName, other }) {
            return {
                slotName: slotName ? 9 : 0,
                other: other ? 10 : 0
            };
        }
        s({ slotName, other }) {
            let w = this.w;
            this._$ctx[0] = slotName;
            this._$ctx[1] = other;
        }
    }
    class CreateSlot15 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            t.n16 = direct_1.createSpan();
            direct_1.setText(t.n16, w.slot + slotName + other);
            return this.n16;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n16, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [slotName, other] = t._$ctx;
            let { dirty0 } = w._$info;
            if (dirty0 & 448) direct_1.setText(t.n16, w.slot + slotName + other);
            return this.n16;
        }
        static getDirty({ slotName, other }) {
            return {
                slotName: slotName ? 7 : 0,
                other: other ? 8 : 0
            };
        }
        s({ slotName, other }) {
            let w = this.w;
            this._$ctx[0] = slotName;
            this._$ctx[1] = other;
        }
    }
    class BW25 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n25 = direct_1.createWidget(vue_vue_1.TestSingleInner1, w, {
                attrs: {
                    clazzName: w.clazzName,
                    className: w.className,
                    color: w.color
                },
                style: {
                    width /*width*/: "100%",
                    backgroundColor /*backgroundColor*/: "rgb(255,255,0)",
                    height /*height*/: (w.height ? w.height : 300) + 'px'
                },
                class: 3412889436,
                events: {
                    "ev-use-evt": $event => {
                        let r = alert('子组件事件');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    },
                    "pointerclick": $event => {
                        let r = w.changeHeight();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            }, w);
            return this.n25;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n25);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n25, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 7) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.clazzName = w.clazzName;
                if (dirty0 & 2) _$attrs.className = w.className;
                if (dirty0 & 4) _$attrs.color = w.color;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 8) {
                const _$style = {};
                if (dirty0 & 8) _$style.height = (w.height ? w.height : 300) + 'px';
                _$wAttrs.style = _$style;
            }
            ;
            direct_1.patchAttrsForWidget(t.n25, _$wAttrs);
            return this.n25;
        }
    }
    exports.BW25 = BW25;
    class BW27 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n27 = direct_1.createDiv();
            direct_1.extendAttr(t.n27, w, null, true);
            t.n28 = direct_1.createDiv();
            direct_1.setStyle(t.n28, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n28, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n28, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n28, 46 /*backgroundColor*/, w.color);
            direct_1.setEvent(t.n28, "pointerclick", $event => {
                let r = w.say('hello');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n29 = direct_1.createDiv();
            direct_1.setClass(t.n29, [2869555723, direct_1.classHash(w.className), 389030959, direct_1.classHash(w.clazzName, w)]);
            return this.n27;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n28, t.n27);
            direct_1.append(t.n29, t.n27);
            direct_1.insertBefore(t.n27, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n27, w);
            if (dirty0 & 1) direct_1.setStyle(t.n28, 46 /*backgroundColor*/, w.color);
            if (dirty0 & 6) direct_1.setClass(t.n29, [2869555723, direct_1.classHash(w.className), 389030959, direct_1.classHash(w.clazzName, w)]);
            return this.n27;
        }
    }
    exports.BW27 = BW27;
    class BW31 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s31() {
            let w = this.w;
            return w.show === 'blue' ? B31 : w.show === 'red' ? B32 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.i31 = t.s31();
            t.n31 = direct_1.createIf(w, t.i31);
            return this.n31;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n31);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            this.parent = target;
            t.n31.m(target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            t.n31 = direct_1.patchIf(w, t.n31, t.i31, t.i31 = t.s31(), this.parent);
            return this.n31;
        }
    }
    exports.BW31 = BW31;
    class B32 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n32 = direct_1.createDiv();
            direct_1.setStyle(t.n32, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n32, 46 /*backgroundColor*/, staticObj[9]);
            direct_1.setStyle(t.n32, 1 /*height*/, w.height + 'px');
            direct_1.setEvent(t.n32, "pointerclick", $event => {
                let r = w.changeIf();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n32, w, null, true);
            return this.n32;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n32, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 2) direct_1.setStyle(t.n32, 1 /*height*/, w.height + 'px');
            direct_1.extendAttr(t.n32, w);
            return this.n32;
        }
    }
    class B31 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n31 = direct_1.createDiv();
            direct_1.setStyle(t.n31, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n31, 46 /*backgroundColor*/, staticObj[8]);
            direct_1.setStyle(t.n31, 1 /*height*/, w.height + 'px');
            direct_1.setEvent(t.n31, "pointerclick", $event => {
                let r = w.changeIf();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n31, w, null, true);
            return this.n31;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n31, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 2) direct_1.setStyle(t.n31, 1 /*height*/, w.height + 'px');
            direct_1.extendAttr(t.n31, w);
            return this.n31;
        }
    }
    class BW34 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n34 = direct_1.createDiv();
            direct_1.extendAttr(t.n34, w, null, true);
            t.n35 = direct_1.createShow(w, B35, w.show);
            t.n36 = direct_1.createSpan();
            direct_1.setStyle(t.n36, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n36, 46 /*backgroundColor*/, staticObj[9]);
            direct_1.setEvent(t.n36, "pointerclick", $event => {
                let r = w.changeShow();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n36, w.show ? "隐藏" : "显示");
            return this.n34;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n35);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n35.m(t.n34);
            direct_1.append(t.n36, t.n34);
            direct_1.insertBefore(t.n34, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n34, w);
            t.n35 = direct_1.patchShow(w, t.n35, B35, w.show);
            if (dirty0 & 1) direct_1.setText(t.n36, w.show ? "隐藏" : "显示");
            return this.n34;
        }
    }
    exports.BW34 = BW34;
    class B35 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n35 = direct_1.createDiv();
            direct_1.setStyle(t.n35, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n35, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n35, 46 /*backgroundColor*/, staticObj[8]);
            return this.n35;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n35, target, anchor);
        }
        p() {
            return this.n35;
        }
    }
    class BW38 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n38 = direct_1.createDiv();
            direct_1.setStyle(t.n38, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n38, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n38, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n38, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setEvent(t.n38, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n38, w, null, true);
            t.n39 = direct_1.createFor(w, w.arr, B39);
            return this.n38;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n39);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n39.m(t.n38);
            direct_1.insertBefore(t.n38, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n38, w);
            direct_1.patchFor(w, t.n39, w.arr, B39);
            return this.n38;
        }
    }
    exports.BW38 = BW38;
    class B39 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s41() {
            let w = this.w;
            let [item] = this._$ctx;
            return item ? B41 : null;
        }
        c() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            t.n40 = direct_1.createDiv();
            direct_1.setStyle(t.n40, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n40, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n40, 46 /*backgroundColor*/, staticObj[8]);
            direct_1.setStyle(t.n40, 16 /*margin*/, staticObj[12]);
            direct_1.setEvent(t.n40, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.i41 = t.s41();
            t.n41 = direct_1.createIf(w, t.i41);
            return this.n40;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n41);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n41.m(t.n40);
            direct_1.insertBefore(t.n40, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            let { dirty0 } = w._$info;
            t.n41 = direct_1.patchIf(w, t.n41, t.i41, t.i41 = t.s41(), t.n40);
            return this.n40;
        }
        s(item) {
            let w = this.w;
            this._$ctx[0] = item;
        }
    }
    class B41 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            t.n41 = direct_1.createSpan();
            direct_1.setText(t.n41, item);
            return this.n41;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n41, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setText(t.n41, item);
            return this.n41;
        }
    }
    class BW43 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n43 = direct_1.createDiv();
            direct_1.setStyle(t.n43, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n43, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n43, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n43, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n43, 7 /*top*/, staticObj[13]);
            direct_1.setEvent(t.n43, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n43, w, null, true);
            t.n44 = direct_1.createFor(w, w.arr, B44);
            return this.n43;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n44);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n44.m(t.n43);
            direct_1.insertBefore(t.n43, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n43, w);
            direct_1.patchFor(w, t.n44, w.arr, B44);
            return this.n43;
        }
    }
    exports.BW43 = BW43;
    class B44 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            t.n44 = direct_1.createDiv();
            direct_1.setAttr(t.n44, "key", item);
            direct_1.setStyle(t.n44, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n44, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n44, 46 /*backgroundColor*/, staticObj[8]);
            direct_1.setStyle(t.n44, 16 /*margin*/, staticObj[12]);
            t.n45 = direct_1.createSpan();
            direct_1.setText(t.n45, item);
            return this.n44;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n45, t.n44);
            direct_1.insertBefore(t.n44, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n44, "key", item);
            direct_1.setText(t.n45, item);
            return this.n44;
        }
        s(item) {
            let w = this.w;
            this._$ctx[0] = item;
        }
        getKey(item) {
            return item;
        }
    }
    class BW47 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n47 = direct_1.createDiv();
            direct_1.setStyle(t.n47, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n47, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n47, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n47, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n47, 7 /*top*/, staticObj[13]);
            direct_1.setEvent(t.n47, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n47, w, null, true);
            t.n48 = direct_1.createSlot(w, {
                other: "xxx",
                slotName: w.name
            }, "default");
            return this.n47;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n48);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n48.m(t.n47);
            direct_1.insertBefore(t.n47, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n47, w);
            var _$param = {};
            if (dirty0 & 1) _$param.slotName = w.name;
            direct_1.patchSlot(t.n48, w, _$param, "default");
            return this.n47;
        }
    }
    exports.BW47 = BW47;
    class BW50 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n50 = direct_1.createDiv();
            direct_1.setStyle(t.n50, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n50, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n50, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n50, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n50, 7 /*top*/, staticObj[13]);
            direct_1.setEvent(t.n50, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n50, w, null, true);
            t.n51 = direct_1.createSlot(w, {
                other: "xxx",
                slotName: w.name
            }, "default", CreateSlot51);
            return this.n50;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n51);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n51.m(t.n50);
            direct_1.insertBefore(t.n50, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n50, w);
            var _$param = {};
            if (dirty0 & 1) _$param.slotName = w.name;
            direct_1.patchSlot(t.n51, w, _$param, "default");
            return this.n50;
        }
    }
    exports.BW50 = BW50;
    class CreateSlot51 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n52 = direct_1.createSpan();
            direct_1.setText(t.n52, w.name + ":默认插槽");
            return this.n52;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n52, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setText(t.n52, w.name + ":默认插槽");
            return this.n52;
        }
    }
    class BW54 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n54 = direct_1.createDiv();
            direct_1.setStyle(t.n54, 0 /*width*/, staticObj[10]);
            direct_1.setStyle(t.n54, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n54, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n54, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n54, 7 /*top*/, staticObj[13]);
            direct_1.setEvent(t.n54, "pointerclick", $event => {
                let r = w.change();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n54, w, null, true);
            t.n55 = direct_1.createSlot(w, {
                other: "xxx",
                slotName: w.name
            }, "default", CreateSlot55);
            return this.n54;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n55);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n55.m(t.n54);
            direct_1.insertBefore(t.n54, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n54, w);
            var _$param = {};
            if (dirty0 & 1) _$param.slotName = w.name;
            direct_1.patchSlot(t.n55, w, _$param, "default");
            return this.n54;
        }
    }
    exports.BW54 = BW54;
    class CreateSlot55 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n55 = direct_1.createTemplate();
            t.n56 = direct_1.createSpan();
            direct_1.setText(t.n56, w.name + ":默认插槽节点1");
            t.n57 = direct_1.createSpan();
            direct_1.setText(t.n57, w.name + ":默认插槽节点2");
            return this.n55;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n56, t.n55);
            direct_1.append(t.n57, t.n55);
            direct_1.insertBefore(t.n55, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setText(t.n56, w.name + ":默认插槽节点1");
            if (dirty0 & 1) direct_1.setText(t.n57, w.name + ":默认插槽节点2");
            return this.n55;
        }
    }
    class BW59 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n59 = direct_1.createDiv();
            direct_1.extendAttr(t.n59, w, null, true);
            return this.n59;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n59, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n59, w);
            return this.n59;
        }
    }
    exports.BW59 = BW59;
    class BW61 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n61 = direct_1.createInput();
            direct_1.setEvent(t.n61, "input", e => {
                w.name = e.source.value;
            });
            direct_1.setAttr(t.n61, "value", w.name);
            direct_1.setStyle(t.n61, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n61, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n61, null /*bacgroundColor*/, "#553333");
            direct_1.setEvent(t.n61, "input", $event => {
                let r = w.emit('change', w.name);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n61, w, null, true);
            return this.n61;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n61, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setAttr(t.n61, "value", w.name);
            direct_1.extendAttr(t.n61, w);
            return this.n61;
        }
    }
    exports.BW61 = BW61;
});