_$pi.define("app_a/util/setup", ["require", "exports", "module", "pi_common/ui/main_root", "pi_common/main", "pi_common/util/db", "pi_sys/index", "pi_common/sdk/index", "pi_utils/util/frame_mgr", "pi_utils/util/task_mgr", "app_a/login/server/login.topic", "./client_date", "app_a/cfg/default_style", "pi_utils/render/index", "pi_gui/engine/tools", "app_a/widget/tips/tips", "../../app_a/widget/cloud/cloud.vue", "pi_common/store/store", "pi_common/constant", "app_a/struct/db_role.struct", "./db_tool", "pi_common/client_net/network", "pi_common/client_net/net_env", "app_a/login/server/user.topic", "pi_utils/util/logger", "pi_common/id_verify/id_verify", "pi_gui/ui/scratch", "pi_common/id_verify/verify.vue", "app_a/login/client/login.vue", "app_a/login/client/register.vue", "pi_common/debug/index", "pi_common/util/hotfix/client", "pi_common/im/server/db/user.struct", "pi_common/im/server/db/message.struct", "./new_store", "pi_common/im/server/db/message.struct", "pi_common/ui/hot_key"], function (require, exports, module, main_root_1, main_1, db, index_1, index_2, frame_mgr_1, task_mgr_1, login_topic_1, client_date_1, default_style_1, index_3, tools_1, tips_1, cloud_vue_1, store_1, constant_1, db_role_struct_1, db_tool_1, network_1, net_env_1, user_topic_1, logger_1, id_verify_1, scratch, verify_vue_1, login_vue_1, register_vue_1, index_4, client_1, user_struct_1, message_struct_1, new_store_1, message_struct_2, hot_key_1) {
    "use strict";

    exports.openLoginTpl = exports.initNetEnv = exports.init = exports.clientRpc = exports.Global = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    var Global;
    (function (Global) {})(Global = exports.Global || (exports.Global = {}));
    // rpc方法调用 
    exports.clientRpc = (name, req, timeout) => {
        return Global.netEnv.rpc(name, req, timeout);
    };
    let loginWidget;
    let clearFunc;
    let composer;
    let lastLoad;
    exports.init = function () {
        try {
            logI("app_a init start");
            index_1.PISYS.Logger.privacyInfoUpload();
            // PISYS.Native.KeyBoard.onKeyDown(e => {
            //     console.log("====== ", e);
            // })
            // const res = new ResTab();
            // res.load(RES_VIDEO, "app_a/Video3.mp4", []).then(r => {
            //     const video = r.link;
            //     console.log("loadVideo success ", video);
            //     video.width = "1000"
            //     video.height = "800"
            //     video.autoplay = true;
            //     video.controls = "controls"
            //     document.body.appendChild(video as any);
            //     setTimeout(() => {
            //         res.clear()
            //     }, 3000);
            // })
            // PISYS.ResLoadTool.loadVideo("app_a/Video3.mp4", []).then(r => {
            //     console.log("loadVideo success ", r);
            //     r.width = "1000"
            //     r.height = "800"
            //     r.autoplay = true;
            //     r.controls = "controls"
            //     document.body.appendChild(r as any);
            // }).catch(e => {
            //     console.log("loadVideo faile ", e);
            // })
            hot_key_1.registerHotKey("Control+a", () => {
                console.log("Control+a 11111111111");
            });
            hot_key_1.registerHotKey("Control+A", () => {
                console.log("Control+a 222222222222");
            });
            hot_key_1.registerHotKey("Control+C", () => {
                console.log("Control+c 1111111111");
            });
            hot_key_1.registerHotKey("Space", () => {
                console.log("Space 1111111111");
            });
            index_1.PISYS.Cursor.setCursor("boot/drops.ico");
            index_1.PISYS.Native.App.hideFullScreenPopupWindow();
            // 展示 日志 刷新 用法；
            console.log("日志刷新开始");
            index_1.PISYS.Logger.flushLog().then(() => {
                console.log("日志刷新完毕");
            }).catch(e => {
                console.log("日志刷新 失败，网络原因或者其他因素，日志丢失了: ", e);
            });
            index_1.PISYS.Module.setModuleScheduler(task_mgr_1.set);
            // 加载剩余资源
            lastLoad = loadLast();
            // // 加载字体
            // await loadFonts();
            let vdocument = window.vdocument;
            // 设置默认字体，gui默认的字体为"arial",应用程序可以修改该默认，比如此处将默认字体设置为"SOURCEHANSANSK-MEDIUM"
            default_style_1.setDefaultFont(vdocument);
            // 打开gui根组件；
            main_root_1.openRoot();
            // 由于本demo需要将ui和场景结合，handleGuiCompose将gui渲染到一个离屏fbo（渲染目标）中，在每个帧循环结束前，将该fbo渲染在已经绘制好场景的canvas上
            // 之所以这么做，是因为ui大部分时候时静态的，gui在界面不发生改变的情况下，默认不会重新渲染，而场景大部分情况下是动态的，需要实时渲染，这里将场景渲染到canvas，并每帧更新。gui渲染到一个离屏fbo，大部分时间保持静止。每帧结束前，将fbo覆盖到canvas上。
            handleGuiCompose();
            if (index_1.PISYS.Env.get("gui_play")) {
                return Promise.resolve(lastLoad.then(() => {
                    removeLogo();
                    index_4.initDebug(); // 如果要播放gui，应该保证gui使用的资源就绪，如字体、图片等
                }));
                ;
            } else {
                // 初始化gui的debug信息（可传入一个包含如上字段的对象给initDebug方法，如果这些字段不存在，则会从env中		读取对应的控制参数）
                index_4.initDebug();
            }
            main_1.PluginManager.use(tips_1.TipsPlugin);
            // 打开登录页面
            exports.openLoginTpl();
            return Promise.resolve(index_2.getSdkContext()).then(function (_index_2$getSdkContex) {
                Global.sdk = _index_2$getSdkContex;
                // 获取授权
                getAuth();
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    // 初始化网络环境
    exports.initNetEnv = function (auth) {
        try {
            return Promise.resolve(lastLoad).then(function () {
                // 已经初始化过
                if (Global.netEnv) {
                    startGame(auth);
                    return;
                }
                initScene();
                Global.netEnv = new net_env_1.NetEnv();
                Global.netEnv.connect(false, () => {
                    doProofreadServerTime();
                    // idVerify(auth);
                    startGame(auth);
                });
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    // 打开登陆页面
    exports.openLoginTpl = () => {
        // loginWidget = open('app_a-login-client-login');
        loginWidget = main_root_1.open(login_vue_1.default);
        if (index_1.PISYS.Env.get('platform_type') != 'minigame') removeLogo();
        // 自动化测试模块
        // import("pi_common/auto_test/auto_test").then(({ run }) => {
        //     run("test");
        // });
    };
    // 开始游戏
    const startGame = auth => {
        // 登录游戏
        logI("Enter startGame");
        new_store_1.newStore.notify('uuid', auth.token.uuid);
        exports.clientRpc(login_topic_1.login, JSON.stringify(auth)).then(r => {
            logD("login success, r = ", r);
            let rid = r.rid;
            // 上报数据
            const userInfo = {
                serverID: "server_1",
                userID: rid,
                uuid: auth.token.uuid
            };
            Global.sdk.upload.login(userInfo);
            // 如果要本地调用rpc，需要把 .event.ts 文件构建到前端资源目录中 dst | dst_release
            let localWhiteRoster = new Set();
            // localWhiteRoster.add(collectMoney);
            let seriesWhiteRoster = new Set();
            seriesWhiteRoster.add(user_topic_1.collectMoney);
            let onlyLocalWhiteRoster = new Set();
            // 初始化网关
            Global.netEnv.initGateway(Global.dbMgr, rid, {
                localWhiteRoster,
                seriesWhiteRoster,
                onlyLocalWhiteRoster
            });
            client_1.activate(network_1.Network.rootClient, network_1.Network.mqtt.rpc);
            cloud_vue_1.openCloud({
                enterCb: () => {
                    const { initSceneAndMeteor } = require('app_b/main_container/main_container.vue');
                    initSceneAndMeteor();
                    loginWidget && main_root_1.close(loginWidget);
                    loginWidget = null;
                    register_vue_1.closeRegister();
                    cloud_vue_1.closeCloud();
                }
            });
            // 获取玩家数据
            exports.clientRpc(login_topic_1.getAllData, null).then(r => {
                logD("clientRpc(getAllData) ok, r = %o", r);
                const data = JSON.parse(r);
                store_1.initStore(Global.dbMgr, rid, [{ tab: db_role_struct_1.CurrencyDb._$info.name, value: db_tool_1.initialTab(db_role_struct_1.CurrencyDb, data.money || { rid }), key: rid, ware: constant_1.WARE_NAME }, { tab: db_role_struct_1.RoleDb._$info.name, value: db_tool_1.initialTab(db_role_struct_1.RoleDb, data.dbRole || { rid }), key: rid, ware: constant_1.WARE_NAME }, { tab: user_struct_1.Contact._$info.name, value: db_tool_1.initialTab(user_struct_1.Contact, data.contact || { uid: rid }), key: rid, ware: constant_1.WARE_NAME }]);
                network_1.Network.sub({
                    subStruct: message_struct_1.GroupMsgLog,
                    primaryKey: `send/group_msg/${data.gid}`,
                    customKey: true,
                    subFn: r => {
                        new_store_1.newStore.notify('groupChat', r);
                    }
                });
                new_store_1.newStore.notify('rid', rid);
                new_store_1.newStore.notify('gid', data.gid);
            }).catch(e => {
                logE('clientRpc getAllData failed, reason = %o', e);
                return Promise.reject(e);
            });
            // 设置默认主键
            network_1.Network.setDefaultPrimaryKey(rid);
            network_1.Network.sub({
                subStruct: db_role_struct_1.CurrencyDb
            });
            network_1.Network.sub({
                subStruct: user_struct_1.Contact
            });
            network_1.Network.sub({
                subStruct: message_struct_2.UserMsgLog,
                primaryKey: `send/c2c_msg/${rid}`,
                customKey: true,
                subFn: r => {
                    // 系统消息 不用关心顺序
                    if (r.key[0] === 0) {
                        let list = new_store_1.newStore.find('sysMessage');
                        list.push(r);
                        new_store_1.newStore.notify('sysMessage', list);
                    } else {
                        new_store_1.newStore.notify('friendChat', r);
                    }
                }
            });
        }).catch(e => {
            logE('login error: %o', e);
        });
    };
    // 加载字体
    const loadFonts = () => {
        return index_1.PISYS.ResLoadTool.loadFont('res_3d/font/SOURCEHANSANSK-MEDIUM.TTF', [{
            family: 'SOURCEHANSANSK-MEDIUM'
        }]);
    };
    // 校对服务器时间
    const doProofreadServerTime = () => {
        exports.clientRpc(login_topic_1.proofreadServerTime, Date.now()).then(proofreadResult => {
            logD("clientRpc ok, proofreadResult = %o", proofreadResult);
            client_date_1.ClientDate.updateTime(proofreadResult.serverTime);
        }).catch(e => {
            logE('doProofreadServerTime error = %o', e);
        });
    };
    // 加载剩余资源
    const loadLast = function () {
        try {
            const lastLoad = index_1.PISYS.Env.get('last_load');
            const load = new index_1.PISYS.BatchLoad.BatchLoad(lastLoad);
            return Promise.resolve(load.start(index_1.PISYS.BatchLoad.LoadPolicy.All, index_1.PISYS.BatchLoad.LoadPolicy.All).then(() => {
                index_1.PISYS.Env.set('loadFinish', true);
            }));
        } catch (e) {
            return Promise.reject(e);
        }
    };
    // 获取授权
    const getAuth = function () {
        try {
            // 初始化基础环境
            let baseEnv = main_1.initBase([{ name: "memory", dbType: db.DbType.Memory }, { name: "logfile", dbType: db.DbType.File }]);
            Global.dbMgr = baseEnv.dbMgr;
            // 模拟后端环境（本地rpc调用需要用到）
            window["env"] = baseEnv.env;
            let { host, port, isTls } = index_1.PISYS.Env.get('platform_server').default;
            Global.sdk.login.setServer(host, port, isTls);
            Global.sdk.login.saveAuth(true);
            Global.sdk.login.useLastAuth(true);
            Global.sdk.login.onLogin((info, fail) => {
                if (fail) {
                    logW("login fail, reason = %o", fail);
                } else {
                    logD("login success, sdkName = %s", info.sdkName);
                    // 开始授权 登录成功后立即授权，一定不会失败
                    Global.sdk.login.getAuth().then(auth => {
                        logD("auth success，auth = %o", auth);
                        exports.initNetEnv(auth);
                    });
                }
            });
            Global.sdk.login.onLogout((info, fail) => {
                if (fail) {
                    logE("logout failed, reason = %o", fail);
                } else {
                    logD("logout success, sdkName = %s" + info.sdkName);
                    // 清除凭证，具体说明看 注释
                    Global.sdk.login.clearAuth();
                }
            });
            // 获取授权，已登录过会保存信息，直接可授权成功
            // Global.sdk.login.getAuth().then(auth => {
            //     logD("auto success, auth = %o", auth);
            //     initNetEnv(auth);
            // }).catch(e => {
            //     logW("auto failed, reason = %o", e);
            // });
            return Promise.resolve();
        } catch (e) {
            return Promise.reject(e);
        }
    };
    // 处理gui和场景合并渲染
    const handleGuiCompose = () => {
        const root = main_root_1.getRoot();
        let canvas = root.canvas;
        let gl = canvas.getContext('webgl');
        // if (!isRegisterLost) {
        // 	isRegisterLost = true;
        // 	if (!gl.getExtension('WEBGL_lose_context')) {
        // 		alert("该宿主没有 WebGL 环境 丢失的扩展");
        // 	}
        // 	canvas.addEventListener('webglcontextrestored', function(e) {
        // 		alert("WebGL 环境已经恢复");	
        // 	}, false);
        // 	canvas.addEventListener('webglcontextlost', function(e) {
        // 		alert("WebGL 环境丢失");
        // 		try {
        // 			let loseExtension = gl.getExtension('WEBGL_lose_context');
        // 			if (loseExtension) {
        // 				loseExtension.restoreContext();
        // 			}
        // 		} catch (e) {
        // 			alert("WebGL 环境 恢复 失败");
        // 		}
        // 	}, false);
        // } 
        // 在小游戏上，需要每帧对整个canvas清屏
        // 清屏的问题需要特别小心，经常会出现框架加载的进度条被删除后，场景初始化完成之前，没有任何地方对canvas清屏，结果造成花屏。
        // 因此这里将清屏单独放入帧循环的最前面。应该注意的是，其它在canvas上渲染的方法不应该再次清屏（如场景，合成器）
        clearFunc = () => {
            gl.bindFramebuffer(gl.FRAMEBUFFER, null);
            gl.enable(gl.SCISSOR_TEST);
            gl.scissor(0, 0, root.canvas.width, canvas.height);
            gl.disable(gl.SCISSOR_TEST);
            gl.clearColor(1, 0, 0, 1);
            gl.clear(gl.COLOR_BUFFER_BIT);
        };
        frame_mgr_1.getGlobal().setPermanentBefore(clearFunc);
        // 创建一个渲染目标，并设置到gui， 使得gui渲染到该渲染目标上
        // 创建一个渲染合成器，并将gui对应的渲染目标加入到合成器中；将合成器的渲染添加到帧循环的末尾，合成器将会每帧将gui的渲染目标合成到canvas上
        let viewPortWidth, viewPortHeight;
        if (root.document.isRotate) {
            viewPortHeight = root.document.viewPortWidth;
            viewPortWidth = root.document.viewPortHeight;
        } else {
            viewPortWidth = root.document.viewPortWidth;
            viewPortHeight = root.document.viewPortHeight;
        }
        composer = new index_3.Composer(gl, canvas.width, canvas.height);
        let renderTarget = composer.createRenderTarget(viewPortWidth, viewPortHeight);
        // vdocument渲染到一个fbo上
        root.document.bindRenderTarget(renderTarget.fbo);
        root.document.setClearColor(new tools_1.RGBA(0, 0, 0, 0));
        // 将该fob添加到渲染合成器上
        composer.add(renderTarget);
        // gui渲染
        frame_mgr_1.getGlobal().setPermanent(composer.render.bind(composer));
        // 处理划痕渲染
        scratch.setRenderTarget(canvas);
        // gui合成到canvas上后，渲染目标已经切换成了canvas，此时正好是将划痕渲染到canvas上的时机
        frame_mgr_1.getGlobal().setPermanent(scratch.render);
        // // //设置渲染帧率
        // getGlobal().setInterval(30); // 华为快游戏设置低于60帧会闪屏
    };
    // 初始化场景（需要等待最后一次下载结束）
    const initScene = () => {
        return Promise.all([new Promise((resolve_1, reject_1) => {
            require(["pi_babylon/scene_base"], resolve_1, reject_1);
        }), new Promise((resolve_2, reject_2) => {
            require(["pi_babylon/scene"], resolve_2, reject_2);
        })]).then(([SceneBase, { SceneManager, SceneManagerData }]) => {
            try {
                SceneBase.ResPath = 'res_3d/';
                SceneManager.init(main_root_1.getRoot().canvas);
                //TODO: engine 没有 clearColor
                SceneManagerData.engine["clearColor"] = null;
                // 场景循环放在清屏之后
                frame_mgr_1.getGlobal().setPermanent(SceneManager.renderLoop, null, clearFunc);
            } catch (error) {
                logE("initScene failed, reason = %o", error);
            }
        });
    };
    // 移除加载时的logo
    const removeLogo = () => {
        document && document.getElementById && document.getElementById('logo') && document.getElementById('logo').remove();
    };
    // 实名认证
    const idVerify = auth => {
        id_verify_1.identityVerification.idVerifySuccess(r => {
            startGame(auth);
        });
        id_verify_1.identityVerification.idVerifyError(r => {
            console.log('id idVerify error', r);
        });
        let { host, port, isTls } = index_1.PISYS.Env.get('platform_server').default;
        id_verify_1.identityVerification.query(host, port, isTls, 'ydzm', auth.token.uuid, Global.netEnv.rpc, true).then(r => {
            // 未实名
            if (!r) {
                main_root_1.open(verify_vue_1.default);
            }
        });
    };
});