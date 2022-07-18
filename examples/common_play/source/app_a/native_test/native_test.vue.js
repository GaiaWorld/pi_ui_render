_$pi.define("app_a/native_test/native_test.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./native_test.vue.tpl", "pi_sys/index", "pi_common/ui/main_root", "app_a/util/setup", "pi_utils/util/logger", "app_c/demo/demo_single.vue"], function (require, exports, module, direct_1, native_test_vue_tpl_1, index_1, main_root_1, setup_1, logger_1, demo_single_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    let albumFileInfo;
    class NativeTest {
        constructor() {
            this.cfglist = cfglist;
            this.quit = () => {
                main_root_1.close(this);
            };
            this.click = i => {
                const cfg = cfglist[i - 0];
                if (cfg) {
                    main_root_1.open(demo_single_vue_1.default, { list: cfg[1] });
                }
            };
        }
    }
    exports.default = NativeTest;
    const userName = `u_${Date.now() % 100000}`;
    const Native = index_1.PISYS.Native;
    const cfglist = [["系统", [["exitApp", () => {
        return new Promise(() => {
            Native.App.exitApp();
        });
    }], ["getChannelInfo", () => {
        return Native.App.getChannelInfo().then(info => {
            return "getChannelInfo: " + JSON.stringify(info);
        });
    }], ["Quick-ChannelType", () => {
        return Native.SpecialCall.getChannelType("quicksdk").then(info => {
            return "ChannelType: " + JSON.stringify(info);
        });
    }], ["Quick-ShowToolbar", () => {
        return Native.SpecialCall.showToolbar("quicksdk").then(info => {
            return "ShowToolbar: " + JSON.stringify(info);
        });
    }], ["Quick-HideToolbar", () => {
        return Native.SpecialCall.hideToolbar("quicksdk").then(info => {
            return "HideToolbar: " + JSON.stringify(info);
        });
    }], ["QuickSDK-verifyRealName", () => {
        return Native.SpecialCall.verifyRealName("quicksdk").then(info => {
            return "verifyRealName: " + JSON.stringify(info);
        });
    }], ["openWebView", () => {
        return new Promise(() => {
            Native.App.openWebView("https://www.baidu.com").then(tag => {
                setTimeout(() => {
                    alert("now call Native.App.closeWebView");
                    Native.App.closeWebView(tag);
                }, 3000);
            });
        });
    }], ["ApiVersion", () => {
        return new Promise(resolve => {
            Native.App.getApiVersion().then(v => {
                resolve(`getApiVersion, ${v}`);
            });
        });
    }], ["PackageName", () => {
        return new Promise(resolve => {
            Native.App.getPackageName().then(v => {
                resolve(`getPackageName, ${v}`);
            });
        });
    }], ["FlavorName", () => {
        return new Promise(resolve => {
            Native.App.getFlavorName().then(v => {
                resolve(`getFlavorName, ${v}`);
            });
        });
    }], ["VersionName", () => {
        return new Promise(resolve => {
            Native.App.getVersionName().then(v => {
                resolve(`getVersionName, ${v}`);
            });
        });
    }], ["VersionCode", () => {
        return new Promise(resolve => {
            Native.App.getVersionCode().then(v => {
                resolve(`getVersionCode, ${v}`);
            });
        });
    }], ["SystemInfo", () => {
        return new Promise(resolve => {
            Native.SystemInfo.getSystemInfo().then(v => {
                resolve(`getSystemInfo, ${JSON.stringify(v)}`);
            });
        });
    }], ["DeviceID", () => {
        return new Promise(resolve => {
            Native.SystemInfo.getDeviceID().then(v => {
                resolve(`getDeviceID, ${v}`);
            });
        });
    }], ["注册APP生命周期监听", () => {
        Native.App.registerAppLifeListener(flag => {
            logI(`AppLife11111111111: ${flag}`);
        });
        Native.App.registerAppLifeListener(flag => {
            logI(`AppLife222222222222222: ${flag}`);
        });
    }], ["取消注册APP生命周期监听", () => {
        Native.App.unregisterAppLifeListener();
    }], ["onShow", () => {
        Native.App.onShow(() => {
            logI("onShow 111111");
        });
        Native.App.onShow(() => {
            logI("onShow 222222");
        });
    }], ["onHide", () => {
        Native.App.onHide(() => {
            logI("onHide 111111");
        });
        Native.App.onHide(() => {
            logI("onHide 222222");
        });
    }], ["offShow", () => {
        Native.App.offShow();
    }], ["offHide", () => {
        Native.App.offHide();
    }]]], ["加速计", [["开始监听", () => {
        Native.Accelerometer.start();
    }], ["停止监听", () => {
        Native.Accelerometer.stop();
    }], ["注册监听", () => {
        Native.Accelerometer.onChange(info => {
            logI("加速计监听1111", JSON.stringify(info));
        });
        Native.Accelerometer.onChange(info => {
            logI("加速计监听2222", JSON.stringify(info));
        });
    }], ["取消注册监听", () => {
        Native.Accelerometer.offChange();
    }]]], ["电量", [["获取电量信息", () => {
        return new Promise(resolve => {
            Native.Battery.getInfo().then(info => {
                resolve(JSON.stringify(info));
                logI("获取电量信息", JSON.stringify(info));
            });
        });
    }], ["设置电量变化监听", () => {
        Native.Battery.onLevelChange(info => {
            logI(`电量变化11111 `, JSON.stringify(info));
        });
        Native.Battery.onLevelChange(info => {
            logI(`电量变化2222 `, JSON.stringify(info));
        });
    }], ["取消电量变化监听", () => {
        Native.Battery.offLevelChange();
    }], ["设置充放电变化监听", () => {
        Native.Battery.onChargingChange(info => {
            logI(`充放电变化11111 `, JSON.stringify(info));
        });
        Native.Battery.onChargingChange(info => {
            logI(`充放电变化2222 `, JSON.stringify(info));
        });
    }], ["取消充放电变化监听", () => {
        Native.Battery.offChargingChange();
    }]]], ["剪切板", [["setData", () => {
        return new Promise((resolve, reject) => {
            Native.Clipboard.setData("Clipboard Test.").then(() => {
                resolve(`Clipboard set OK`);
            }).catch(reject);
        });
    }], ["getData", () => {
        return new Promise((resolve, reject) => {
            Native.Clipboard.getData().then(text => {
                resolve(`Clipboard get OK - ${text}`);
            }).catch(reject);
        });
    }]]], ["罗盘", [["开始监听", () => {
        Native.Compass.start();
    }], ["停止监听", () => {
        Native.Compass.stop();
    }], ["注册监听", () => {
        Native.Compass.onChange(info => {
            logI("罗盘监听1111", JSON.stringify(info));
        });
        Native.Compass.onChange(info => {
            logI("罗盘监听2222", JSON.stringify(info));
        });
    }], ["取消注册监听", () => {
        Native.Compass.offChange();
    }]]], ["陀螺仪", [["开始监听", () => {
        Native.Gyroscope.start();
    }], ["停止监听", () => {
        Native.Gyroscope.stop();
    }], ["注册监听", () => {
        Native.Gyroscope.onChange(info => {
            logI("陀螺仪监听1111", JSON.stringify(info));
        });
        Native.Gyroscope.onChange(info => {
            logI("陀螺仪监听2222", JSON.stringify(info));
        });
    }], ["取消注册监听", () => {
        Native.Gyroscope.offChange();
    }]]], ["定位", [["获取位置信息", () => {
        return new Promise(resolve => {
            Native.Location.getLocation("wgs84", "true", true, 4000).then(level => {
                resolve(`Gyroscope ${JSON.stringify(level)}`);
                logI("定位信息", JSON.stringify(level));
            }).catch(e => {
                resolve(`Gyroscope err`);
                logI("定位信息 err", JSON.stringify(e));
            });
        });
    }], ["开始监听", () => {
        Native.Location.start();
    }], ["停止监听", () => {
        Native.Location.stop();
    }], ["注册监听", () => {
        Native.Location.onChange(info => {
            logI("定位监听1111", JSON.stringify(info));
        });
        Native.Location.onChange(info => {
            logI("定位监听2222", JSON.stringify(info));
        });
    }], ["取消注册监听", () => {
        Native.Location.offChange();
    }]]], ["内存性能", [["注册监听", () => {
        Native.MemoryWarning.onWarning(level => {
            logI("内存性能监听", level);
        });
    }], ["取消注册监听", () => {
        Native.MemoryWarning.offWarning();
    }]]], ["设备方向", [["开始监听", () => {
        Native.DeviceMotion.start();
    }], ["停止监听", () => {
        Native.DeviceMotion.stop();
    }], ["注册监听", () => {
        Native.DeviceMotion.onChange(info => {
            logI("设备方向监听1111", JSON.stringify(info));
        });
        Native.DeviceMotion.onChange(info => {
            logI("设备方向监听2222", JSON.stringify(info));
        });
    }], ["取消注册监听", () => {
        Native.DeviceMotion.offChange();
    }]]], ["网络", [["获取网络类型", () => {
        return new Promise(resolve => {
            Native.Network.getType().then(level => {
                resolve(level);
                logI("获取网络类型", level);
            });
        });
    }], ["监听网络变化", () => {
        Native.Network.onStatusChange(level => {
            logI("网络变化111111111-----------", level);
        });
        Native.Network.onStatusChange(level => {
            logI("网络变化22222222-----------", level);
        });
    }], ["取消监听网络变化", () => {
        Native.Network.offStatusChange();
    }]]], ["横竖屏", [["监听横竖屏切换", () => {
        Native.DeviceOrientation.onChange(v => {
            logI("横竖屏切换11111", v);
        });
        Native.DeviceOrientation.onChange(v => {
            logI("横竖屏切换22222", v);
        });
    }], ["取消监听横竖屏切换", () => {
        Native.DeviceOrientation.offChange();
    }]]], ["二维码", [["扫码", () => {
        return new Promise((resolve, reject) => {
            Native.QRCode.scanCode().then(level => {
                resolve(`扫描二维码 - ${level}`);
            }).catch(reject);
        });
    }]]], ["屏幕", [["获取屏幕亮度", () => {
        return new Promise(resolve => {
            Native.Screen.getBrightness().then(level => {
                resolve(JSON.stringify(level));
            });
        });
    }], ["设置屏幕亮度", () => {
        return new Promise(resolve => {
            Native.Screen.setBrightness(0.5).then(level => {
                resolve(level);
            });
        });
    }], ["保持常亮", () => {
        return new Promise(resolve => {
            Native.Screen.setKeepOn(true).then(level => {
                resolve(level);
            });
        });
    }], ["监听截屏事件", () => {
        Native.Screen.onUserCaptureScreen(level => {
            logI("截屏监听111111", level);
        });
    }], ["取消监听截屏事件", () => {
        Native.Screen.offUserCaptureScreen();
    }]]], ["震动", [["短震", () => {
        Native.Vibrate.vibrateShort("heavy");
    }], ["长震", () => {
        Native.Vibrate.vibrateLong();
    }]]], ["对话框", [["提示框", () => {
        Native.Dialog.showToast("提示框框框框框", undefined, undefined, 5000);
        // setTimeout(hideToast, 2000);
    }], ["关闭提示框", () => {
        Native.Dialog.hideToast();
    }], ["模态对话框", () => {
        Native.Dialog.showModel("模态标题", "模态内容", true, "消去按钮", "#ff0000", "认确按钮", "#00ff00");
    }], ["loading提示框", () => {
        Native.Dialog.showLoading("loading标题", true);
        setTimeout(Native.Dialog.hideLoading, 1000);
    }], ["隐藏loading提示框", () => {
        Native.Dialog.hideLoading();
    }]]], ["相册", [["选择图片", function () {
        try {
            return Promise.resolve(Native.Image.chooseImage(9, Native.Image.SizeType.Compressed, Native.Image.SourceType.Camera)).then(JSON.stringify);
        } catch (e) {
            return Promise.reject(e);
        }
    }]]], ["相机", [["打开相册选择相片", () => {
        return Native.Camera.openAlbum("image").then(r => {
            console.error("===== 打开相册选择相片 Success", r);
            albumFileInfo = r[0];
        });
    }], ["打开相册选择视频", () => {
        return Native.Camera.openAlbum("video").then(r => {
            console.error("===== 打开相册选择视频 Success", r);
        });
    }], ["打开相机 拍照 ", () => {
        return Native.Camera.openCamera("image").then(r => {
            console.error("===== 打开相机 拍照 Success", r);
        });
    }], ["打开相机 录像 ", () => {
        return Native.Camera.openCamera("video").then(r => {
            console.error("===== 打开相机 录像 Success", r);
        });
    }], ["扫码", () => {
        return Native.Camera.scanQRCode().then(r => {
            console.error("===== 扫码 Success", r);
        });
    }], ["保存 图片 到相册", () => {
        let path = "";
        return Native.Camera.save2Album(path).then(r => {
            console.error("===== 保存 图片 到相册 Success", r);
        });
    }], ["获取图片内容", () => {
        return Native.Camera.getBase64FromLocalFile(albumFileInfo.path).then(r => {
            console.error("===== 获取图片内容 Success", r.length);
        });
    }]]], ["移动推送监听", [["阿里云移动推送", function () {
        try {
            // 打开监听
            Native.Push.aliyun();
            // 设置uuid和设备绑定
            return Promise.resolve(Native.SpecialCall.aliyun_bindAccount("uuid")).then(function () {
                // 调用后端接口推送数据
                return Promise.resolve(setup_1.clientRpc("app_c/demo/pi_common/server/pi_common.templateApp", "")).then(function () {
                    return "ok";
                });
            });
        } catch (e) {
            return Promise.reject(e);
        }
    }]]], ["音视频通讯", [["login", () => {
        return new Promise((resolve, reject) => {
            // 调用后端接口推送数据
            setup_1.clientRpc("app_c/demo/pi_common/server/pi_common.genSig", userName).then(r => {
                Native.VoIPChat.login(1400429313, userName, JSON.stringify(r)).then(level => {
                    resolve(`VoIPChat login - ${level}`);
                }).catch(reject);
            });
        });
    }], ["joinVideoRoom", () => {
        Native.VoIPChat.joinVideoRoom(999, userName, "c", true, true, 2);
    }], ["join 语音", () => {
        Native.VoIPChat.join("999", false, false);
    }], ["exit 语音", () => {
        Native.VoIPChat.exit();
    }], ["update 语音", () => {
        Native.VoIPChat.updateMuteConfig(true, false);
    }], ["监听 语音成员加入/退出", () => {
        return new Promise(resolve => {
            Native.VoIPChat.registerMembersChanged(level => {
                resolve(`VoIPChat registerMembersChanged - ${JSON.stringify(level)}`);
            });
        });
    }], ["监听 语音成员开始/停止说话", () => {
        return new Promise(resolve => {
            Native.VoIPChat.registerSpeakersChanged(level => {
                resolve(`VoIPChat registerSpeakersChanged - ${JSON.stringify(level)}`);
            });
        });
    }], ["获取实时音视频房间列表", () => {
        // 调用后端接口推送数据
        return setup_1.clientRpc("app_c/demo/pi_common/server/pi_common.trtc").then(r => {
            return "trtc:" + r;
        });
    }]]], ["项目配置设置", [["获取用户设置", () => {
        return new Promise(resolve => {
            const cfg = index_1.PISYS.Env.get("userConfig");
            resolve("userConfig " + JSON.stringify(cfg));
        });
    }], ["设置低配", () => {
        const cfg = {
            engine: {
                benchLevel: index_1.PISYS.Native.SystemInfo.BenchLevel.Low
            }
        };
        // 设置后需要刷新重进才能生效
        index_1.PISYS.Store.setUserConfig(cfg);
    }], ["设置中配", () => {
        const cfg = {
            engine: {
                benchLevel: index_1.PISYS.Native.SystemInfo.BenchLevel.Middle
            }
        };
        index_1.PISYS.Store.setUserConfig(cfg);
    }], ["设置高配", () => {
        const cfg = {
            engine: {
                benchLevel: index_1.PISYS.Native.SystemInfo.BenchLevel.High
            }
        };
        index_1.PISYS.Store.setUserConfig(cfg);
    }]]], ["广告", [["加载广告", () => {
        index_1.PISYS.Native.Ad.load("reward").then(r => {
            console.log("加载广告成功", r);
        });
    }], ["播放广告", () => {
        index_1.PISYS.Native.Ad.show("reward").then(r => {
            console.log("播放广告成功", r);
        });
    }], ["注册回调", () => {
        index_1.PISYS.Native.Ad.onClose("reward", r => {
            console.log("关闭回调 = ", r);
        });
    }]]], ["键盘", [["监听键盘按下", () => {
        index_1.PISYS.Native.KeyBoard.onKeyDown(param => {
            console.log("键盘按下 code / key / timeStamp", param.code, param.key, param.timeStamp);
        });
    }], ["监听键盘弹起", () => {
        index_1.PISYS.Native.KeyBoard.onKeyUp(param => {
            console.log("键盘弹起 code / key / timeStamp", param.code, param.key, param.timeStamp);
        });
    }]]]];
    exports.initMeta = () => {
        let _$tpl = "app_a/native_test/native_test.vue.tpl.ts",
            _$cssPath = "app_a/native_test/native_test.vue.wcss",
            _$cssHash = 2449264234;
        NativeTest["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: native_test_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(NativeTest, ["cfglist"]);
    direct_1.addField(NativeTest, ['quit', 'click']);
});