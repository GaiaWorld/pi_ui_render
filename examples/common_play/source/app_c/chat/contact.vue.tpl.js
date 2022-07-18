_$pi.define("app_c/chat/contact.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./sys_msg_item.vue"], function (require, exports, module, direct_1, sys_msg_item_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 10]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 1061581821, true);
            t.n3 = direct_1.createDiv();
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.goBack();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 574499091);
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 2808685192);
            direct_1.setText(t.n4, "返回");
            t.n5 = direct_1.createDiv();
            t.n6 = direct_1.createSpan();
            direct_1.setClass(t.n6, 2808685192);
            direct_1.setText(t.n6, "好友列表");
            t.n7 = direct_1.createFor(w, w.contact.friends, B7);
            t.n9 = direct_1.createFor(w, w.sysMsg, B9);
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 17 /*marginTop*/, staticObj[0]);
            t.n12 = direct_1.createSpan();
            direct_1.setClass(t.n12, 2808685192);
            direct_1.setText(t.n12, "对方rid");
            t.n13 = direct_1.createDiv();
            direct_1.setClass(t.n13, 2297265776);
            t.n14 = direct_1.createInput();
            direct_1.setAttr(t.n14, "type", "text");
            direct_1.setAttr(t.n14, "value", w.rid);
            direct_1.setEvent(t.n14, "change", $event => {
                let r = w.inputRid($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n14, 3536357503);
            t.n15 = direct_1.createSpan();
            direct_1.setEvent(t.n15, "pointerclick", $event => {
                let r = w.addUser();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n15, 4151959596);
            direct_1.setText(t.n15, "加好友");
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n9);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            t.n7.m(t.n2);
            t.n9.m(t.n2);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n11, t.n2);
            direct_1.append(t.n14, t.n13);
            direct_1.append(t.n13, t.n2);
            direct_1.append(t.n15, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1061581821);
            direct_1.patchFor(w, t.n7, w.contact.friends, B7);
            direct_1.patchFor(w, t.n9, w.sysMsg, B9);
            if (dirty0 & 4) direct_1.setAttr(t.n14, "value", w.rid);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B9 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            t.n10 = direct_1.createWidget(sys_msg_item_vue_1.default, w, {
                attrs: {
                    msgLog: item,
                    key: item.key
                },
                events: {
                    "ev-deal": $event => {
                        let r = w.deal($event);
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
            let [item] = t._$ctx;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.msgLog = item;
                _$attrs.key = item.key;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n10, _$wAttrs);
            return this.n10;
        }
        s(item) {
            let w = this.w;
            this._$ctx[0] = item;
        }
    }
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [item, i] = t._$ctx;
            t.n7 = direct_1.createDiv();
            direct_1.setAttr(t.n7, "key", i);
            t.n8 = direct_1.createSpan();
            direct_1.setClass(t.n8, 2808685192);
            direct_1.setText(t.n8, item);
            return this.n7;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n8, t.n7);
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [item, i] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n7, "key", i);
            direct_1.setText(t.n8, item);
            return this.n7;
        }
        s(item, i) {
            let w = this.w;
            this._$ctx[0] = item;
            this._$ctx[1] = i;
        }
        getKey(item, i) {
            return i;
        }
    }
});