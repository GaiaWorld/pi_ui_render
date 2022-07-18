_$pi.define("app_c/demo/ui/menu/root.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./root.vue.tpl", "pi_common/ui/main_root", "pi_sys/index", "pi_utils/util/logger", "./treebtn.vue"], function (require, exports, module, direct_1, root_vue_tpl_1, main_root_1, index_1, logger_1, treebtn_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    // ============================== 导出
    /**
     * @description 导出组件Widget类
     * @example
     */
    class Root {
        constructor() {
            this.sid = 0;
            this.btnWidget = treebtn_vue_1.default;
        }
        /**
         * @description 设置属性，默认外部传入的props是完整的props，重载可改变行为
         * @example
         */
        propsUpdate() {
            console.log("dd");
            // const btn = relative('treebtn$', this.name);
            // const arr = list();
            const tree = { arr: [], count: 0, show: { select: true, sid: ++this.sid } };
            //root形如“show-”
            let modules = window["_$pi"]._modules;
            let initVueCom = () => {
                for (let m of modules) {
                    let name = m[0];
                    if (name.endsWith(".vue") && name.startsWith(this.showDir)) {
                        this.sid += putNode(tree, name.slice(this.showDir.length, name.length - 4).split("/"), name, ++this.sid);
                    }
                }
            };
            initVueCom();
            for (const n of tree.arr) {
                mergeAndSortNode(n);
            }
            this.tree = tree;
            this.show = true;
            this.widget = null;
            // logD("tree=============", tree);
            // this.props = { tree: tree, widget: null, show: true };
        }
        // changeOpen(widget: string){
        // 	this.widget = widget;
        // }
        /**
         * @description 打开指定的组件
         * @example
         */
        open(widget) {
            this.widget = index_1.PISYS.Module.requireSync(widget).default;
            ;
        }
        showMenu() {
            this.show = !this.show;
        }
        quit() {
            main_root_1.close(this);
        }
    }
    exports.default = Root;
    // ============================== 本地
    // 寻找节点
    const find = (arr, name) => {
        for (let i = 0; i < arr.length; i++) {
            if (arr[i].show.cfg.text === name) {
                return i;
            }
        }
        return -1;
    };
    // 放置节点
    const putNode = (tree, names, widget, sid) => {
        for (let i = 0; i < names.length; i++) {
            const s = names[i];
            if (!tree.arr) {
                tree.arr = [];
                tree.show.leaf = false;
            }
            tree.count++;
            const index = find(tree.arr, s);
            if (index < 0) {
                const n = { cmd: widget, show: { cfg: { text: s }, leaf: true, select: false, sid: ++sid }, name: widget, arr: null, count: 0 };
                tree.arr.push(n);
                tree = n;
            } else {
                tree = tree.arr[index];
            }
        }
        return sid;
    };
    // 合并单个子元素的节点
    const mergeAndSortNode = node => {
        if (!node.arr) {
            return;
        }
        if (node.arr.length === 1) {
            const n = node.arr[0];
            n.show.cfg.text = `${node.show.cfg.text}-${n.show.cfg.text}`;
            node.show = n.show;
            node.arr = n.arr;
            return mergeAndSortNode(node);
        }
        for (const n of node.arr) {
            mergeAndSortNode(n);
        }
        node.arr = node.arr.sort((a, b) => {
            var _a, _b, _c, _d;
            return ((_b = (_a = a.show) === null || _a === void 0 ? void 0 : _a.cfg) === null || _b === void 0 ? void 0 : _b.text) > ((_d = (_c = b.show) === null || _c === void 0 ? void 0 : _c.cfg) === null || _d === void 0 ? void 0 : _d.text) ? 1 : -1;
        });
    };
    // ============================== 立即执行
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/menu/root.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/menu/root.vue.wcss",
            _$cssHash = 3940191281;
        Root["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: root_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Root, ["widget", "show", "tree", "btnWidget"]);
    direct_1.addField(Root, ['showDir', 'sub']);
});