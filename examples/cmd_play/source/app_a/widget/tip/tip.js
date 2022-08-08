_$pi.define("app_a/widget/tip/tip", ["require", "exports", "module", "pi_gui/widget/forelet", "pi_gui/widget/widget", "pi_utils/util/task_mgr"], function (require, exports, module, forelet_1, widget_1, task_mgr_1) {
    "use strict";

    exports.TipManager = exports.Tip = exports.forelet = void 0;
    exports.forelet = new forelet_1.Forelet();
    class Tip extends widget_1.Widget {
        setProps(props) {
            super.setProps(props);
            this.init();
        }
        setState(state) {
            super.setState(state);
            this.init();
        }
        init() {
            if (!this.props.conditionKey) return;
            this.props.show = this.props.show || 'tip_default';
        }
    }
    exports.Tip = Tip;
    // 红点管理器，所有使用的红点都需要在此注册
    var TipManager;
    (function (TipManager) {
        let listen;
        let dbCacheMap = new Map();
        /**
         * ex: app_a/store下的register
         * @param listenFunc 前端监听数据库变化的方法
         */
        function setDBListener(listenFunc) {
            listen = listenFunc;
        }
        TipManager.setDBListener = setDBListener;
        /**
         *
         * @param dbStruct 监听的数据库名，当该数据库发生变动时红点状态更新
         * @param conditionKey 和在tpl中传的conditionKey参数保持一致作为更新唯一表示
         * @param conditionFunc 红点状态判断函数，当状态发生变化时被调用，会将当前监听的数据库和已在红点中监听的其它数据库map传入
         */
        function register(dbStruct, conditionKey, conditionFunc) {
            if (!listen) throw Error('TipManager.setDBListener function must call before register!');
            listen(dbStruct, db => {
                dbCacheMap.set(dbStruct, db);
                let redPoint = conditionFunc && conditionFunc(db, dbCacheMap);
                let paintFunc = () => {
                    let widget = exports.forelet.widgets.find(w => w.props.conditionKey == conditionKey);
                    widget.state.redPoint = redPoint;
                    widget.setProps(widget.props);
                    widget.paint();
                };
                task_mgr_1.set(paintFunc, [], 900000, 1);
            });
        }
        TipManager.register = register;
    })(TipManager = exports.TipManager || (exports.TipManager = {}));
});