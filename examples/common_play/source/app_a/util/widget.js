_$pi.define("app_a/util/widget", ["require", "exports", "module", "pi_sys/index", "pi_gui/widget/widget", "pi_gui/widget/painter", "pi_gui/widget/vnode"], function (require, exports, module, index_1, widget_1, painter_1, vnode_1) {
    "use strict";

    exports.RootWidget = exports.Override = void 0;
    // 用于重写
    exports.Override = (target, name, descriptor) => {
        const oldFunc = descriptor.value;
        const proto = Object.getPrototypeOf(target);
        const callFuncList = [];
        let protoFunc;
        if (target instanceof RootWidget) {
            protoFunc = Object.getPrototypeOf(proto)[name];
        } else {
            protoFunc = proto[name];
        }
        callFuncList.push(protoFunc, oldFunc);
        descriptor.value = function () {
            callFuncList.forEach(fn => fn.apply(this));
        };
    };
    class RootWidget extends widget_1.Widget {
        setProps(props) {
            const newProps = Object.assign(Object.assign({}, this.props), props);
            return super.setProps(newProps);
        }
        getNodeById(id) {
            return painter_1.getRealNode(vnode_1.findNodeByAttr(this.tree, 'id', id));
        }
        getNodeByTag(tag, value) {
            return painter_1.getRealNode(vnode_1.findNodeByAttr(this.tree, tag, value));
        }
        attach() {
            // 配合底部菜单使用
            const { ScreenEvent, SCREEN_HANDLER } = index_1.PISYS.Module.requireSync('app_b/main_container/main_container');
            super.attach();
            SCREEN_HANDLER.notify(ScreenEvent.WidgetPainted, [this.name]);
        }
        paint(rest) {
            if (this.document === undefined) return;
            super.paint(rest);
        }
    }
    exports.RootWidget = RootWidget;
});