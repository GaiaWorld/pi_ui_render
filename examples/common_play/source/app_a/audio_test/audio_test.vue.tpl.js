_$pi.define("app_a/audio_test/audio_test.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 100]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 2676862099, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            t.n4 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: '退出'
                },
                class: 3583788939,
                events: {
                    "pointerclick": $event => {
                        let r = w.quit($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n5 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: w.flag ? '静音' : '播放'
                },
                class: 3583788939,
                events: {
                    "pointerclick": $event => {
                        let r = w.trigger($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 3382833726);
            t.n7 = direct_1.createSpan();
            direct_1.setClass(t.n7, 1501291140);
            direct_1.setText(t.n7, "bgm music");
            t.n8 = direct_1.createFor(w, w.bgmList, B8);
            t.n9 = direct_1.createDiv();
            direct_1.setClass(t.n9, 3382833726);
            t.n10 = direct_1.createSpan();
            direct_1.setClass(t.n10, 1501291140);
            direct_1.setText(t.n10, "sound music");
            t.n11 = direct_1.createFor(w, w.soundList, B11);
            t.n12 = direct_1.createDiv();
            direct_1.setClass(t.n12, 3382833726);
            t.n13 = direct_1.createSpan();
            direct_1.setClass(t.n13, 1501291140);
            direct_1.setText(t.n13, "InnerAudio Test");
            t.n14 = direct_1.createFor(w, w.innerTestList, B14);
            t.n15 = direct_1.createDiv();
            direct_1.setClass(t.n15, 3382833726);
            t.n16 = direct_1.createSpan();
            direct_1.setClass(t.n16, 1501291140);
            direct_1.setText(t.n16, "Audio Mgr Test");
            t.n17 = direct_1.createFor(w, w.soundMgrList, B17);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n11);
            direct_1.destroyContext(t.n14);
            direct_1.destroyContext(t.n17);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, t.n3);
            direct_1.mountChildWidget(t.n5, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n7, t.n6);
            t.n8.m(t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.append(t.n10, t.n9);
            t.n11.m(t.n9);
            direct_1.append(t.n9, t.n2);
            direct_1.append(t.n13, t.n12);
            t.n14.m(t.n12);
            direct_1.append(t.n12, t.n2);
            direct_1.append(t.n16, t.n15);
            t.n17.m(t.n15);
            direct_1.append(t.n15, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 2676862099);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.text = w.flag ? '静音' : '播放';
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            direct_1.patchFor(w, t.n8, w.bgmList, B8);
            direct_1.patchFor(w, t.n11, w.soundList, B11);
            direct_1.patchFor(w, t.n14, w.innerTestList, B14);
            direct_1.patchFor(w, t.n17, w.soundMgrList, B17);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B17 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n17 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    key: i,
                    text: v
                },
                class: 3583788939,
                events: {
                    "ev-click": $event => {
                        let i = t.$i;
                        let r = w.soundMgr(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
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
            this.$i = i;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = i;
                _$attrs.text = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n17, _$wAttrs);
            return this.n17;
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
    class B14 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n14 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    key: i,
                    text: v
                },
                class: 3583788939,
                events: {
                    "ev-click": $event => {
                        let i = t.$i;
                        let r = w.innerTest(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n14;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n14);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n14, target, anchor);
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
                _$attrs.text = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n14, _$wAttrs);
            return this.n14;
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
    class B11 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n11 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    key: i,
                    text: v
                },
                class: 3583788939,
                events: {
                    "ev-click": $event => {
                        let i = t.$i;
                        let r = w.playAudio(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n11;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n11);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n11, target, anchor);
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
                _$attrs.text = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n11, _$wAttrs);
            return this.n11;
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
    class B8 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            this.$i = i;
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    key: i,
                    text: v
                },
                class: 3583788939,
                events: {
                    "ev-click": $event => {
                        let i = t.$i;
                        let r = w.playBGM(i);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n8;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n8);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n8, target, anchor);
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
                _$attrs.text = v;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n8, _$wAttrs);
            return this.n8;
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