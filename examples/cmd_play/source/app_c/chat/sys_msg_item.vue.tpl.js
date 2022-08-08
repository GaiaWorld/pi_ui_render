_$pi.define("app_c/chat/sys_msg_item.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.msgLog ? B3 : null;
        }

        s7() {
            let w = this.w;
            return w.showBtn ? B7 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 256703858, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.i7 = t.s7();
            t.n7 = direct_1.createIf(w, t.i7);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n7);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n7.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 256703858);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            t.n7 = direct_1.patchIf(w, t.n7, t.i7, t.i7 = t.s7(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n7 = direct_1.createTemplate();
            t.n8 = direct_1.createSpan();
            direct_1.setEvent(t.n8, "pointerclick", $event => {
                let r = w.accept();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n8, 3204555575);
            direct_1.setText(t.n8, "同意");
            t.n9 = direct_1.createSpan();
            direct_1.setEvent(t.n9, "pointerclick", $event => {
                let r = w.refuse();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n9, 3204555575);
            direct_1.setText(t.n9, "拒绝");
            return this.n7;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n8, t.n7);
            direct_1.append(t.n9, t.n7);
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            return this.n7;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createTemplate();
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 3462981207);
            direct_1.setText(t.n4, "用户：" + w.send);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 3462981207);
            direct_1.setText(t.n5, "内容：" + w.mess);
            t.n6 = direct_1.createSpan();
            direct_1.setClass(t.n6, 3462981207);
            direct_1.setText(t.n6, "时间：" + w.time);
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n6, t.n3);
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 2) direct_1.setText(t.n4, "用户：" + w.send);
            if (dirty0 & 4) direct_1.setText(t.n5, "内容：" + w.mess);
            if (dirty0 & 8) direct_1.setText(t.n6, "时间：" + w.time);
            return this.n3;
        }
    }
});