_$pi.define("app_a/download/download.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./download.vue.tpl", "pi_sys/index", "pi_common/ui/main_root"], function (require, exports, module, direct_1, download_vue_tpl_1, index_1, main_root_1) {
    "use strict";

    exports.initMeta = exports.DownloadManager = void 0;
    let downLoadWid;
    class Download {
        constructor() {
            this.processTxt = '';
        }
        create() {
            this.processTxt = '';
            downLoadWid = this;
        }
        destroy() {
            downLoadWid = undefined;
        }
        updateProgress(process) {
            const p = Math.min(Math.round(process * 100), 100);
            if (isNaN(p)) return;
            this.processTxt = `${p}%`;
        }
    }
    exports.default = Download;
    //加载管理器
    var DownloadManager;
    (function (DownloadManager) {
        const downloadFunc = files => {
            const fileLoader = new index_1.PISYS.BatchLoad.BatchLoad(files);
            fileLoader.addProcess((_fileName, _fileType, total, loaded) => {
                downLoadWid.updateProgress(loaded / total);
            });
            return new Promise(resolve => {
                fileLoader.addResult(() => {
                    main_root_1.close(downLoadWid);
                    resolve('');
                });
                fileLoader.start(2, 2);
            });
        };
        function download(files) {
            main_root_1.open(Download);
            return downloadFunc(files);
        }
        DownloadManager.download = download;
    })(DownloadManager = exports.DownloadManager || (exports.DownloadManager = {}));
    exports.initMeta = () => {
        let _$tpl = "app_a/download/download.vue.tpl.ts",
            _$cssPath = "app_a/download/download.vue.wcss",
            _$cssHash = 1842800692;
        Download["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: download_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Download, ["processTxt"]);
    direct_1.addField(Download, ['ok']);
});