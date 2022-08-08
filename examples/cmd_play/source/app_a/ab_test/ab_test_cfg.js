_$pi.define("app_a/ab_test/ab_test_cfg", ["require", "exports", "module", "pi_common/ab_test/server/ab_test", "pi_utils/util/logger"], function (require, exports, module, ABTest, logger_1) {
    "use strict";

    exports.getTag = exports.init = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    const abTestCfg = [{
        id: '1', cfg: {
            name: 'pi_demo',
            startTime: Date.now() - 1000000,
            endTime: Date.now() + 1000000,
            group: [{
                tag: 'tag1',
                limit: '',
                percent: 10
            }, {
                tag: 'tag2',
                limit: '',
                percent: 30
            }, {
                tag: 'tag3',
                limit: '',
                percent: 70
            }]
        }
    }];
    exports.init = () => {
        ABTest.cfgUpload(abTestCfg, '1');
    };
    exports.getTag = function (uid) {
        try {
            return Promise.resolve(ABTest.getUserTag(env.dbMgr, abTestCfg, uid, '1')).then(function (r) {
                logD('!!!!!!!!!!!!get ABtest Tag:', r);
                return r;
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
});