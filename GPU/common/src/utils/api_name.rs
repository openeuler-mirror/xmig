// SPDX-License-Identifier: Mulan PSL v2
/*
 * Copyright (c) 2025 Huawei Technologies Co., Ltd.
 * This software is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *         http://license.coscl.org.cn/MulanPSL2
 *
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiFuncName {
    /// cudaDeviceReset
    FuncCudadevicereset = 0,
    /// cudaDeviceSynchronize
    FuncCudadevicesynchronize = 1,
    /// cudaDeviceSetLimit
    FuncCudadevicesetlimit = 2,
    /// cudaDeviceGetLimit
    FuncCudadevicegetlimit = 3,
    /// cudaDeviceGetTexture1DLinearMaxWidth
    FuncCudadevicegettexture1dlinearmaxwidth = 4,
    /// cudaDeviceGetCacheConfig
    FuncCudadevicegetcacheconfig = 5,
    /// cudaDeviceGetStreamPriorityRange
    FuncCudadevicegetstreampriorityrange = 6,
    /// cudaDeviceSetCacheConfig
    FuncCudadevicesetcacheconfig = 7,
    /// cudaDeviceGetByPCIBusId
    FuncCudadevicegetbypcibusid = 8,
    /// cudaDeviceGetPCIBusId
    FuncCudadevicegetpcibusid = 9,
    /// cudaIpcGetEventHandle
    FuncCudaipcgeteventhandle = 10,
    /// cudaIpcOpenEventHandle
    FuncCudaipcopeneventhandle = 11,
    /// cudaIpcGetMemHandle
    FuncCudaipcgetmemhandle = 12,
    /// cudaIpcOpenMemHandle
    FuncCudaipcopenmemhandle = 13,
    /// cudaIpcCloseMemHandle
    FuncCudaipcclosememhandle = 14,
    /// cudaDeviceFlushGPUDirectRDMAWrites
    FuncCudadeviceflushgpudirectrdmawrites = 15,
    /// cudaDeviceRegisterAsyncNotification
    FuncCudadeviceregisterasyncnotification = 16,
    /// cudaDeviceUnregisterAsyncNotification
    FuncCudadeviceunregisterasyncnotification = 17,
    /// cudaDeviceGetSharedMemConfig
    FuncCudadevicegetsharedmemconfig = 18,
    /// cudaDeviceSetSharedMemConfig
    FuncCudadevicesetsharedmemconfig = 19,
    /// cudaThreadExit
    FuncCudathreadexit = 20,
    /// cudaThreadSynchronize
    FuncCudathreadsynchronize = 21,
    /// cudaThreadSetLimit
    FuncCudathreadsetlimit = 22,
    /// cudaThreadGetLimit
    FuncCudathreadgetlimit = 23,
    /// cudaThreadGetCacheConfig
    FuncCudathreadgetcacheconfig = 24,
    /// cudaThreadSetCacheConfig
    FuncCudathreadsetcacheconfig = 25,
    /// cudaGetLastError
    FuncCudagetlasterror = 26,
    /// cudaPeekAtLastError
    FuncCudapeekatlasterror = 27,
    /// cudaGetErrorName
    FuncCudageterrorname = 28,
    /// cudaGetErrorString
    FuncCudageterrorstring = 29,
    /// cudaGetDeviceCount
    FuncCudagetdevicecount = 30,
    /// cudaGetDeviceProperties_v2
    FuncCudagetdevicepropertiesV2 = 31,
    /// cudaDeviceGetAttribute
    FuncCudadevicegetattribute = 32,
    /// cudaDeviceGetDefaultMemPool
    FuncCudadevicegetdefaultmempool = 33,
    /// cudaDeviceSetMemPool
    FuncCudadevicesetmempool = 34,
    /// cudaDeviceGetMemPool
    FuncCudadevicegetmempool = 35,
    /// cudaDeviceGetNvSciSyncAttributes
    FuncCudadevicegetnvscisyncattributes = 36,
    /// cudaDeviceGetP2PAttribute
    FuncCudadevicegetp2pattribute = 37,
    /// cudaChooseDevice
    FuncCudachoosedevice = 38,
    /// cudaInitDevice
    FuncCudainitdevice = 39,
    /// cudaSetDevice
    FuncCudasetdevice = 40,
    /// cudaGetDevice
    FuncCudagetdevice = 41,
    /// cudaSetValidDevices
    FuncCudasetvaliddevices = 42,
    /// cudaSetDeviceFlags
    FuncCudasetdeviceflags = 43,
    /// cudaGetDeviceFlags
    FuncCudagetdeviceflags = 44,
    /// cudaStreamCreate
    FuncCudastreamcreate = 45,
    /// cudaStreamCreateWithFlags
    FuncCudastreamcreatewithflags = 46,
    /// cudaStreamCreateWithPriority
    FuncCudastreamcreatewithpriority = 47,
    /// cudaStreamGetPriority
    FuncCudastreamgetpriority = 48,
    /// cudaStreamGetFlags
    FuncCudastreamgetflags = 49,
    /// cudaStreamGetId
    FuncCudastreamgetid = 50,
    /// cudaCtxResetPersistingL2Cache
    FuncCudactxresetpersistingl2cache = 51,
    /// cudaStreamCopyAttributes
    FuncCudastreamcopyattributes = 52,
    /// cudaStreamGetAttribute
    FuncCudastreamgetattribute = 53,
    /// cudaStreamSetAttribute
    FuncCudastreamsetattribute = 54,
    /// cudaStreamDestroy
    FuncCudastreamdestroy = 55,
    /// cudaStreamWaitEvent
    FuncCudastreamwaitevent = 56,
    /// cudaStreamAddCallback
    FuncCudastreamaddcallback = 57,
    /// cudaStreamSynchronize
    FuncCudastreamsynchronize = 58,
    /// cudaStreamQuery
    FuncCudastreamquery = 59,
    /// cudaStreamAttachMemAsync
    FuncCudastreamattachmemasync = 60,
    /// cudaStreamBeginCapture
    FuncCudastreambegincapture = 61,
    /// cudaStreamBeginCaptureToGraph
    FuncCudastreambegincapturetograph = 62,
    /// cudaThreadExchangeStreamCaptureMode
    FuncCudathreadexchangestreamcapturemode = 63,
    /// cudaStreamEndCapture
    FuncCudastreamendcapture = 64,
    /// cudaStreamIsCapturing
    FuncCudastreamiscapturing = 65,
    /// cudaStreamGetCaptureInfo_v2
    FuncCudastreamgetcaptureinfoV2 = 66,
    /// cudaStreamGetCaptureInfo_v3
    FuncCudastreamgetcaptureinfoV3 = 67,
    /// cudaStreamUpdateCaptureDependencies
    FuncCudastreamupdatecapturedependencies = 68,
    /// cudaStreamUpdateCaptureDependencies_v2
    FuncCudastreamupdatecapturedependenciesV2 = 69,
    /// cudaEventCreate
    FuncCudaeventcreate = 70,
    /// cudaEventCreateWithFlags
    FuncCudaeventcreatewithflags = 71,
    /// cudaEventRecord
    FuncCudaeventrecord = 72,
    /// cudaEventRecordWithFlags
    FuncCudaeventrecordwithflags = 73,
    /// cudaEventQuery
    FuncCudaeventquery = 74,
    /// cudaEventSynchronize
    FuncCudaeventsynchronize = 75,
    /// cudaEventDestroy
    FuncCudaeventdestroy = 76,
    /// cudaEventElapsedTime
    FuncCudaeventelapsedtime = 77,
    /// cudaImportExternalMemory
    FuncCudaimportexternalmemory = 78,
    /// cudaExternalMemoryGetMappedBuffer
    FuncCudaexternalmemorygetmappedbuffer = 79,
    /// cudaExternalMemoryGetMappedMipmappedArray
    FuncCudaexternalmemorygetmappedmipmappedarray = 80,
    /// cudaDestroyExternalMemory
    FuncCudadestroyexternalmemory = 81,
    /// cudaImportExternalSemaphore
    FuncCudaimportexternalsemaphore = 82,
    /// cudaSignalExternalSemaphoresAsync_v2
    FuncCudasignalexternalsemaphoresasyncV2 = 83,
    /// cudaWaitExternalSemaphoresAsync_v2
    FuncCudawaitexternalsemaphoresasyncV2 = 84,
    /// cudaDestroyExternalSemaphore
    FuncCudadestroyexternalsemaphore = 85,
    /// cudaLaunchKernel
    FuncCudalaunchkernel = 86,
    /// cudaLaunchKernelExC
    FuncCudalaunchkernelexc = 87,
    /// cudaLaunchCooperativeKernel
    FuncCudalaunchcooperativekernel = 88,
    /// cudaLaunchCooperativeKernelMultiDevice
    FuncCudalaunchcooperativekernelmultidevice = 89,
    /// cudaFuncSetCacheConfig
    FuncCudafuncsetcacheconfig = 90,
    /// cudaFuncSetAttribute
    FuncCudafuncsetattribute = 91,
    /// cudaFuncGetAttributes
    FuncCudafuncgetattributes = 92,
    /// cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags
    FuncCudaoccupancymaxactiveblockspermultiprocessorwithflags = 93,
    /// cudaFuncGetParamInfo
    FuncCudafuncgetparaminfo = 94,
    /// cudaSetDoubleForDevice
    FuncCudasetdoublefordevice = 95,
    /// cudaSetDoubleForHost
    FuncCudasetdoubleforhost = 96,
    /// cudaLaunchHostFunc
    FuncCudalaunchhostfunc = 97,
    /// cudaFuncSetSharedMemConfig
    FuncCudafuncsetsharedmemconfig = 98,
    /// cudaOccupancyMaxActiveBlocksPerMultiprocessor
    FuncCudaoccupancymaxactiveblockspermultiprocessor = 99,
    /// cudaOccupancyAvailableDynamicSMemPerBlock
    FuncCudaoccupancyavailabledynamicsmemperblock = 100,
    /// cudaOccupancyMaxPotentialClusterSize
    FuncCudaoccupancymaxpotentialclustersize = 101,
    /// cudaOccupancyMaxActiveClusters
    FuncCudaoccupancymaxactiveclusters = 102,
    /// cudaMallocManaged
    FuncCudamallocmanaged = 103,
    /// cudaMalloc
    FuncCudamalloc = 104,
    /// cudaMallocHost
    FuncCudamallochost = 105,
    /// cudaMallocPitch
    FuncCudamallocpitch = 106,
    /// cudaMallocArray
    FuncCudamallocarray = 107,
    /// cudaFree
    FuncCudafree = 108,
    /// cudaFreeHost
    FuncCudafreehost = 109,
    /// cudaFreeArray
    FuncCudafreearray = 110,
    /// cudaFreeMipmappedArray
    FuncCudafreemipmappedarray = 111,
    /// cudaHostAlloc
    FuncCudahostalloc = 112,
    /// cudaHostRegister
    FuncCudahostregister = 113,
    /// cudaHostUnregister
    FuncCudahostunregister = 114,
    /// cudaHostGetDevicePointer
    FuncCudahostgetdevicepointer = 115,
    /// cudaHostGetFlags
    FuncCudahostgetflags = 116,
    /// cudaMalloc3D
    FuncCudamalloc3d = 117,
    /// cudaMalloc3DArray
    FuncCudamalloc3darray = 118,
    /// cudaMallocMipmappedArray
    FuncCudamallocmipmappedarray = 119,
    /// cudaGetMipmappedArrayLevel
    FuncCudagetmipmappedarraylevel = 120,
    /// cudaMemcpy3D
    FuncCudamemcpy3d = 121,
    /// cudaMemcpy3DPeer
    FuncCudamemcpy3dpeer = 122,
    /// cudaMemcpy3DAsync
    FuncCudamemcpy3dasync = 123,
    /// cudaMemcpy3DPeerAsync
    FuncCudamemcpy3dpeerasync = 124,
    /// cudaMemGetInfo
    FuncCudamemgetinfo = 125,
    /// cudaArrayGetInfo
    FuncCudaarraygetinfo = 126,
    /// cudaArrayGetPlane
    FuncCudaarraygetplane = 127,
    /// cudaArrayGetMemoryRequirements
    FuncCudaarraygetmemoryrequirements = 128,
    /// cudaMipmappedArrayGetMemoryRequirements
    FuncCudamipmappedarraygetmemoryrequirements = 129,
    /// cudaArrayGetSparseProperties
    FuncCudaarraygetsparseproperties = 130,
    /// cudaMipmappedArrayGetSparseProperties
    FuncCudamipmappedarraygetsparseproperties = 131,
    /// cudaMemcpy
    FuncCudamemcpy = 132,
    /// cudaMemcpyPeer
    FuncCudamemcpypeer = 133,
    /// cudaMemcpy2D
    FuncCudamemcpy2d = 134,
    /// cudaMemcpy2DToArray
    FuncCudamemcpy2dtoarray = 135,
    /// cudaMemcpy2DFromArray
    FuncCudamemcpy2dfromarray = 136,
    /// cudaMemcpy2DArrayToArray
    FuncCudamemcpy2darraytoarray = 137,
    /// cudaMemcpyToSymbol
    FuncCudamemcpytosymbol = 138,
    /// cudaMemcpyFromSymbol
    FuncCudamemcpyfromsymbol = 139,
    /// cudaMemcpyAsync
    FuncCudamemcpyasync = 140,
    /// cudaMemcpyPeerAsync
    FuncCudamemcpypeerasync = 141,
    /// cudaMemcpy2DAsync
    FuncCudamemcpy2dasync = 142,
    /// cudaMemcpy2DToArrayAsync
    FuncCudamemcpy2dtoarrayasync = 143,
    /// cudaMemcpy2DFromArrayAsync
    FuncCudamemcpy2dfromarrayasync = 144,
    /// cudaMemcpyToSymbolAsync
    FuncCudamemcpytosymbolasync = 145,
    /// cudaMemcpyFromSymbolAsync
    FuncCudamemcpyfromsymbolasync = 146,
    /// cudaMemset
    FuncCudamemset = 147,
    /// cudaMemset2D
    FuncCudamemset2d = 148,
    /// cudaMemset3D
    FuncCudamemset3d = 149,
    /// cudaMemsetAsync
    FuncCudamemsetasync = 150,
    /// cudaMemset2DAsync
    FuncCudamemset2dasync = 151,
    /// cudaMemset3DAsync
    FuncCudamemset3dasync = 152,
    /// cudaGetSymbolAddress
    FuncCudagetsymboladdress = 153,
    /// cudaGetSymbolSize
    FuncCudagetsymbolsize = 154,
    /// cudaMemPrefetchAsync
    FuncCudamemprefetchasync = 155,
    /// cudaMemPrefetchAsync_v2
    FuncCudamemprefetchasyncV2 = 156,
    /// cudaMemAdvise
    FuncCudamemadvise = 157,
    /// cudaMemAdvise_v2
    FuncCudamemadviseV2 = 158,
    /// cudaMemRangeGetAttribute
    FuncCudamemrangegetattribute = 159,
    /// cudaMemRangeGetAttributes
    FuncCudamemrangegetattributes = 160,
    /// cudaMemcpyToArray
    FuncCudamemcpytoarray = 161,
    /// cudaMemcpyFromArray
    FuncCudamemcpyfromarray = 162,
    /// cudaMemcpyArrayToArray
    FuncCudamemcpyarraytoarray = 163,
    /// cudaMemcpyToArrayAsync
    FuncCudamemcpytoarrayasync = 164,
    /// cudaMemcpyFromArrayAsync
    FuncCudamemcpyfromarrayasync = 165,
    /// cudaMallocAsync
    FuncCudamallocasync = 166,
    /// cudaFreeAsync
    FuncCudafreeasync = 167,
    /// cudaMemPoolTrimTo
    FuncCudamempooltrimto = 168,
    /// cudaMemPoolSetAttribute
    FuncCudamempoolsetattribute = 169,
    /// cudaMemPoolGetAttribute
    FuncCudamempoolgetattribute = 170,
    /// cudaMemPoolSetAccess
    FuncCudamempoolsetaccess = 171,
    /// cudaMemPoolGetAccess
    FuncCudamempoolgetaccess = 172,
    /// cudaMemPoolCreate
    FuncCudamempoolcreate = 173,
    /// cudaMemPoolDestroy
    FuncCudamempooldestroy = 174,
    /// cudaMallocFromPoolAsync
    FuncCudamallocfrompoolasync = 175,
    /// cudaMemPoolExportToShareableHandle
    FuncCudamempoolexporttoshareablehandle = 176,
    /// cudaMemPoolImportFromShareableHandle
    FuncCudamempoolimportfromshareablehandle = 177,
    /// cudaMemPoolExportPointer
    FuncCudamempoolexportpointer = 178,
    /// cudaMemPoolImportPointer
    FuncCudamempoolimportpointer = 179,
    /// cudaPointerGetAttributes
    FuncCudapointergetattributes = 180,
    /// cudaDeviceCanAccessPeer
    FuncCudadevicecanaccesspeer = 181,
    /// cudaDeviceEnablePeerAccess
    FuncCudadeviceenablepeeraccess = 182,
    /// cudaDeviceDisablePeerAccess
    FuncCudadevicedisablepeeraccess = 183,
    /// cudaGraphicsUnregisterResource
    FuncCudagraphicsunregisterresource = 184,
    /// cudaGraphicsResourceSetMapFlags
    FuncCudagraphicsresourcesetmapflags = 185,
    /// cudaGraphicsMapResources
    FuncCudagraphicsmapresources = 186,
    /// cudaGraphicsUnmapResources
    FuncCudagraphicsunmapresources = 187,
    /// cudaGraphicsResourceGetMappedPointer
    FuncCudagraphicsresourcegetmappedpointer = 188,
    /// cudaGraphicsSubResourceGetMappedArray
    FuncCudagraphicssubresourcegetmappedarray = 189,
    /// cudaGraphicsResourceGetMappedMipmappedArray
    FuncCudagraphicsresourcegetmappedmipmappedarray = 190,
    /// cudaGetChannelDesc
    FuncCudagetchanneldesc = 191,
    /// cudaCreateChannelDesc
    FuncCudacreatechanneldesc = 192,
    /// cudaCreateTextureObject
    FuncCudacreatetextureobject = 193,
    /// cudaDestroyTextureObject
    FuncCudadestroytextureobject = 194,
    /// cudaGetTextureObjectResourceDesc
    FuncCudagettextureobjectresourcedesc = 195,
    /// cudaGetTextureObjectTextureDesc
    FuncCudagettextureobjecttexturedesc = 196,
    /// cudaGetTextureObjectResourceViewDesc
    FuncCudagettextureobjectresourceviewdesc = 197,
    /// cudaCreateSurfaceObject
    FuncCudacreatesurfaceobject = 198,
    /// cudaDestroySurfaceObject
    FuncCudadestroysurfaceobject = 199,
    /// cudaGetSurfaceObjectResourceDesc
    FuncCudagetsurfaceobjectresourcedesc = 200,
    /// cudaDriverGetVersion
    FuncCudadrivergetversion = 201,
    /// cudaRuntimeGetVersion
    FuncCudaruntimegetversion = 202,
    /// cudaGraphCreate
    FuncCudagraphcreate = 203,
    /// cudaGraphAddKernelNode
    FuncCudagraphaddkernelnode = 204,
    /// cudaGraphKernelNodeGetParams
    FuncCudagraphkernelnodegetparams = 205,
    /// cudaGraphKernelNodeSetParams
    FuncCudagraphkernelnodesetparams = 206,
    /// cudaGraphKernelNodeCopyAttributes
    FuncCudagraphkernelnodecopyattributes = 207,
    /// cudaGraphKernelNodeGetAttribute
    FuncCudagraphkernelnodegetattribute = 208,
    /// cudaGraphKernelNodeSetAttribute
    FuncCudagraphkernelnodesetattribute = 209,
    /// cudaGraphAddMemcpyNode
    FuncCudagraphaddmemcpynode = 210,
    /// cudaGraphAddMemcpyNodeToSymbol
    FuncCudagraphaddmemcpynodetosymbol = 211,
    /// cudaGraphAddMemcpyNodeFromSymbol
    FuncCudagraphaddmemcpynodefromsymbol = 212,
    /// cudaGraphAddMemcpyNode1D
    FuncCudagraphaddmemcpynode1d = 213,
    /// cudaGraphMemcpyNodeGetParams
    FuncCudagraphmemcpynodegetparams = 214,
    /// cudaGraphMemcpyNodeSetParams
    FuncCudagraphmemcpynodesetparams = 215,
    /// cudaGraphMemcpyNodeSetParamsToSymbol
    FuncCudagraphmemcpynodesetparamstosymbol = 216,
    /// cudaGraphMemcpyNodeSetParamsFromSymbol
    FuncCudagraphmemcpynodesetparamsfromsymbol = 217,
    /// cudaGraphMemcpyNodeSetParams1D
    FuncCudagraphmemcpynodesetparams1d = 218,
    /// cudaGraphAddMemsetNode
    FuncCudagraphaddmemsetnode = 219,
    /// cudaGraphMemsetNodeGetParams
    FuncCudagraphmemsetnodegetparams = 220,
    /// cudaGraphMemsetNodeSetParams
    FuncCudagraphmemsetnodesetparams = 221,
    /// cudaGraphAddHostNode
    FuncCudagraphaddhostnode = 222,
    /// cudaGraphHostNodeGetParams
    FuncCudagraphhostnodegetparams = 223,
    /// cudaGraphHostNodeSetParams
    FuncCudagraphhostnodesetparams = 224,
    /// cudaGraphAddChildGraphNode
    FuncCudagraphaddchildgraphnode = 225,
    /// cudaGraphChildGraphNodeGetGraph
    FuncCudagraphchildgraphnodegetgraph = 226,
    /// cudaGraphAddEmptyNode
    FuncCudagraphaddemptynode = 227,
    /// cudaGraphAddEventRecordNode
    FuncCudagraphaddeventrecordnode = 228,
    /// cudaGraphEventRecordNodeGetEvent
    FuncCudagrapheventrecordnodegetevent = 229,
    /// cudaGraphEventRecordNodeSetEvent
    FuncCudagrapheventrecordnodesetevent = 230,
    /// cudaGraphAddEventWaitNode
    FuncCudagraphaddeventwaitnode = 231,
    /// cudaGraphEventWaitNodeGetEvent
    FuncCudagrapheventwaitnodegetevent = 232,
    /// cudaGraphEventWaitNodeSetEvent
    FuncCudagrapheventwaitnodesetevent = 233,
    /// cudaGraphAddExternalSemaphoresSignalNode
    FuncCudagraphaddexternalsemaphoressignalnode = 234,
    /// cudaGraphExternalSemaphoresSignalNodeGetParams
    FuncCudagraphexternalsemaphoressignalnodegetparams = 235,
    /// cudaGraphExternalSemaphoresSignalNodeSetParams
    FuncCudagraphexternalsemaphoressignalnodesetparams = 236,
    /// cudaGraphAddExternalSemaphoresWaitNode
    FuncCudagraphaddexternalsemaphoreswaitnode = 237,
    /// cudaGraphExternalSemaphoresWaitNodeGetParams
    FuncCudagraphexternalsemaphoreswaitnodegetparams = 238,
    /// cudaGraphExternalSemaphoresWaitNodeSetParams
    FuncCudagraphexternalsemaphoreswaitnodesetparams = 239,
    /// cudaGraphAddMemAllocNode
    FuncCudagraphaddmemallocnode = 240,
    /// cudaGraphMemAllocNodeGetParams
    FuncCudagraphmemallocnodegetparams = 241,
    /// cudaGraphAddMemFreeNode
    FuncCudagraphaddmemfreenode = 242,
    /// cudaGraphMemFreeNodeGetParams
    FuncCudagraphmemfreenodegetparams = 243,
    /// cudaDeviceGraphMemTrim
    FuncCudadevicegraphmemtrim = 244,
    /// cudaDeviceGetGraphMemAttribute
    FuncCudadevicegetgraphmemattribute = 245,
    /// cudaDeviceSetGraphMemAttribute
    FuncCudadevicesetgraphmemattribute = 246,
    /// cudaGraphClone
    FuncCudagraphclone = 247,
    /// cudaGraphNodeFindInClone
    FuncCudagraphnodefindinclone = 248,
    /// cudaGraphNodeGetType
    FuncCudagraphnodegettype = 249,
    /// cudaGraphGetNodes
    FuncCudagraphgetnodes = 250,
    /// cudaGraphGetRootNodes
    FuncCudagraphgetrootnodes = 251,
    /// cudaGraphGetEdges
    FuncCudagraphgetedges = 252,
    /// cudaGraphGetEdges_v2
    FuncCudagraphgetedgesV2 = 253,
    /// cudaGraphNodeGetDependencies
    FuncCudagraphnodegetdependencies = 254,
    /// cudaGraphNodeGetDependencies_v2
    FuncCudagraphnodegetdependenciesV2 = 255,
    /// cudaGraphNodeGetDependentNodes
    FuncCudagraphnodegetdependentnodes = 256,
    /// cudaGraphNodeGetDependentNodes_v2
    FuncCudagraphnodegetdependentnodesV2 = 257,
    /// cudaGraphAddDependencies
    FuncCudagraphadddependencies = 258,
    /// cudaGraphAddDependencies_v2
    FuncCudagraphadddependenciesV2 = 259,
    /// cudaGraphRemoveDependencies
    FuncCudagraphremovedependencies = 260,
    /// cudaGraphRemoveDependencies_v2
    FuncCudagraphremovedependenciesV2 = 261,
    /// cudaGraphDestroyNode
    FuncCudagraphdestroynode = 262,
    /// cudaGraphInstantiate
    FuncCudagraphinstantiate = 263,
    /// cudaGraphInstantiateWithFlags
    FuncCudagraphinstantiatewithflags = 264,
    /// cudaGraphInstantiateWithParams
    FuncCudagraphinstantiatewithparams = 265,
    /// cudaGraphExecGetFlags
    FuncCudagraphexecgetflags = 266,
    /// cudaGraphExecKernelNodeSetParams
    FuncCudagraphexeckernelnodesetparams = 267,
    /// cudaGraphExecMemcpyNodeSetParams
    FuncCudagraphexecmemcpynodesetparams = 268,
    /// cudaGraphExecMemcpyNodeSetParamsToSymbol
    FuncCudagraphexecmemcpynodesetparamstosymbol = 269,
    /// cudaGraphExecMemcpyNodeSetParamsFromSymbol
    FuncCudagraphexecmemcpynodesetparamsfromsymbol = 270,
    /// cudaGraphExecMemcpyNodeSetParams1D
    FuncCudagraphexecmemcpynodesetparams1d = 271,
    /// cudaGraphExecMemsetNodeSetParams
    FuncCudagraphexecmemsetnodesetparams = 272,
    /// cudaGraphExecHostNodeSetParams
    FuncCudagraphexechostnodesetparams = 273,
    /// cudaGraphExecChildGraphNodeSetParams
    FuncCudagraphexecchildgraphnodesetparams = 274,
    /// cudaGraphExecEventRecordNodeSetEvent
    FuncCudagraphexeceventrecordnodesetevent = 275,
    /// cudaGraphExecEventWaitNodeSetEvent
    FuncCudagraphexeceventwaitnodesetevent = 276,
    /// cudaGraphExecExternalSemaphoresSignalNodeSetParams
    FuncCudagraphexecexternalsemaphoressignalnodesetparams = 277,
    /// cudaGraphExecExternalSemaphoresWaitNodeSetParams
    FuncCudagraphexecexternalsemaphoreswaitnodesetparams = 278,
    /// cudaGraphNodeSetEnabled
    FuncCudagraphnodesetenabled = 279,
    /// cudaGraphNodeGetEnabled
    FuncCudagraphnodegetenabled = 280,
    /// cudaGraphExecUpdate
    FuncCudagraphexecupdate = 281,
    /// cudaGraphUpload
    FuncCudagraphupload = 282,
    /// cudaGraphLaunch
    FuncCudagraphlaunch = 283,
    /// cudaGraphExecDestroy
    FuncCudagraphexecdestroy = 284,
    /// cudaGraphDestroy
    FuncCudagraphdestroy = 285,
    /// cudaGraphDebugDotPrint
    FuncCudagraphdebugdotprint = 286,
    /// cudaUserObjectCreate
    FuncCudauserobjectcreate = 287,
    /// cudaUserObjectRetain
    FuncCudauserobjectretain = 288,
    /// cudaUserObjectRelease
    FuncCudauserobjectrelease = 289,
    /// cudaGraphRetainUserObject
    FuncCudagraphretainuserobject = 290,
    /// cudaGraphReleaseUserObject
    FuncCudagraphreleaseuserobject = 291,
    /// cudaGraphAddNode
    FuncCudagraphaddnode = 292,
    /// cudaGraphAddNode_v2
    FuncCudagraphaddnodeV2 = 293,
    /// cudaGraphNodeSetParams
    FuncCudagraphnodesetparams = 294,
    /// cudaGraphExecNodeSetParams
    FuncCudagraphexecnodesetparams = 295,
    /// cudaGraphConditionalHandleCreate
    FuncCudagraphconditionalhandlecreate = 296,
    /// cudaGetDriverEntryPoint
    FuncCudagetdriverentrypoint = 297,
    /// cudaGetDriverEntryPointByVersion
    FuncCudagetdriverentrypointbyversion = 298,
    /// cudaGetExportTable
    FuncCudagetexporttable = 299,
    /// cudaGetFuncBySymbol
    FuncCudagetfuncbysymbol = 300,
    /// cudaGetKernel
    FuncCudagetkernel = 301,
    /// cuGetErrorString
    FuncCugeterrorstring = 302,
    /// cuGetErrorName
    FuncCugeterrorname = 303,
    /// cuInit
    FuncCuinit = 304,
    /// cuDriverGetVersion
    FuncCudrivergetversion = 305,
    /// cuDeviceGet
    FuncCudeviceget = 306,
    /// cuDeviceGetCount
    FuncCudevicegetcount = 307,
    /// cuDeviceGetName
    FuncCudevicegetname = 308,
    /// cuDeviceGetUuid
    FuncCudevicegetuuid = 309,
    /// cuDeviceGetUuid_v2
    FuncCudevicegetuuidV2 = 310,
    /// cuDeviceGetLuid
    FuncCudevicegetluid = 311,
    /// cuDeviceTotalMem_v2
    FuncCudevicetotalmemV2 = 312,
    /// cuDeviceGetTexture1DLinearMaxWidth
    FuncCudevicegettexture1dlinearmaxwidth = 313,
    /// cuDeviceGetAttribute
    FuncCudevicegetattribute = 314,
    /// cuDeviceGetNvSciSyncAttributes
    FuncCudevicegetnvscisyncattributes = 315,
    /// cuDeviceSetMemPool
    FuncCudevicesetmempool = 316,
    /// cuDeviceGetMemPool
    FuncCudevicegetmempool = 317,
    /// cuDeviceGetDefaultMemPool
    FuncCudevicegetdefaultmempool = 318,
    /// cuDeviceGetExecAffinitySupport
    FuncCudevicegetexecaffinitysupport = 319,
    /// cuFlushGPUDirectRDMAWrites
    FuncCuflushgpudirectrdmawrites = 320,
    /// cuDeviceGetProperties
    FuncCudevicegetproperties = 321,
    /// cuDeviceComputeCapability
    FuncCudevicecomputecapability = 322,
    /// cuDevicePrimaryCtxRetain
    FuncCudeviceprimaryctxretain = 323,
    /// cuDevicePrimaryCtxRelease_v2
    FuncCudeviceprimaryctxreleaseV2 = 324,
    /// cuDevicePrimaryCtxSetFlags_v2
    FuncCudeviceprimaryctxsetflagsV2 = 325,
    /// cuDevicePrimaryCtxGetState
    FuncCudeviceprimaryctxgetstate = 326,
    /// cuDevicePrimaryCtxReset_v2
    FuncCudeviceprimaryctxresetV2 = 327,
    /// cuCtxCreate_v2
    FuncCuctxcreateV2 = 328,
    /// cuCtxCreate_v3
    FuncCuctxcreateV3 = 329,
    /// cuCtxCreate_v4
    FuncCuctxcreateV4 = 330,
    /// cuCtxDestroy_v2
    FuncCuctxdestroyV2 = 331,
    /// cuCtxPushCurrent_v2
    FuncCuctxpushcurrentV2 = 332,
    /// cuCtxPopCurrent_v2
    FuncCuctxpopcurrentV2 = 333,
    /// cuCtxSetCurrent
    FuncCuctxsetcurrent = 334,
    /// cuCtxGetCurrent
    FuncCuctxgetcurrent = 335,
    /// cuCtxGetDevice
    FuncCuctxgetdevice = 336,
    /// cuCtxGetFlags
    FuncCuctxgetflags = 337,
    /// cuCtxSetFlags
    FuncCuctxsetflags = 338,
    /// cuCtxGetId
    FuncCuctxgetid = 339,
    /// cuCtxSynchronize
    FuncCuctxsynchronize = 340,
    /// cuCtxSetLimit
    FuncCuctxsetlimit = 341,
    /// cuCtxGetLimit
    FuncCuctxgetlimit = 342,
    /// cuCtxGetCacheConfig
    FuncCuctxgetcacheconfig = 343,
    /// cuCtxSetCacheConfig
    FuncCuctxsetcacheconfig = 344,
    /// cuCtxGetApiVersion
    FuncCuctxgetapiversion = 345,
    /// cuCtxGetStreamPriorityRange
    FuncCuctxgetstreampriorityrange = 346,
    /// cuCtxResetPersistingL2Cache
    FuncCuctxresetpersistingl2cache = 347,
    /// cuCtxGetExecAffinity
    FuncCuctxgetexecaffinity = 348,
    /// cuCtxRecordEvent
    FuncCuctxrecordevent = 349,
    /// cuCtxWaitEvent
    FuncCuctxwaitevent = 350,
    /// cuCtxAttach
    FuncCuctxattach = 351,
    /// cuCtxDetach
    FuncCuctxdetach = 352,
    /// cuCtxGetSharedMemConfig
    FuncCuctxgetsharedmemconfig = 353,
    /// cuCtxSetSharedMemConfig
    FuncCuctxsetsharedmemconfig = 354,
    /// cuModuleLoad
    FuncCumoduleload = 355,
    /// cuModuleLoadData
    FuncCumoduleloaddata = 356,
    /// cuModuleLoadDataEx
    FuncCumoduleloaddataex = 357,
    /// cuModuleLoadFatBinary
    FuncCumoduleloadfatbinary = 358,
    /// cuModuleUnload
    FuncCumoduleunload = 359,
    /// cuModuleGetLoadingMode
    FuncCumodulegetloadingmode = 360,
    /// cuModuleGetFunction
    FuncCumodulegetfunction = 361,
    /// cuModuleGetFunctionCount
    FuncCumodulegetfunctioncount = 362,
    /// cuModuleEnumerateFunctions
    FuncCumoduleenumeratefunctions = 363,
    /// cuModuleGetGlobal_v2
    FuncCumodulegetglobalV2 = 364,
    /// cuLinkCreate_v2
    FuncCulinkcreateV2 = 365,
    /// cuLinkAddData_v2
    FuncCulinkadddataV2 = 366,
    /// cuLinkAddFile_v2
    FuncCulinkaddfileV2 = 367,
    /// cuLinkComplete
    FuncCulinkcomplete = 368,
    /// cuLinkDestroy
    FuncCulinkdestroy = 369,
    /// cuModuleGetTexRef
    FuncCumodulegettexref = 370,
    /// cuModuleGetSurfRef
    FuncCumodulegetsurfref = 371,
    /// cuLibraryLoadData
    FuncCulibraryloaddata = 372,
    /// cuLibraryLoadFromFile
    FuncCulibraryloadfromfile = 373,
    /// cuLibraryUnload
    FuncCulibraryunload = 374,
    /// cuLibraryGetKernel
    FuncCulibrarygetkernel = 375,
    /// cuLibraryGetKernelCount
    FuncCulibrarygetkernelcount = 376,
    /// cuLibraryEnumerateKernels
    FuncCulibraryenumeratekernels = 377,
    /// cuLibraryGetModule
    FuncCulibrarygetmodule = 378,
    /// cuKernelGetFunction
    FuncCukernelgetfunction = 379,
    /// cuKernelGetLibrary
    FuncCukernelgetlibrary = 380,
    /// cuLibraryGetGlobal
    FuncCulibrarygetglobal = 381,
    /// cuLibraryGetManaged
    FuncCulibrarygetmanaged = 382,
    /// cuLibraryGetUnifiedFunction
    FuncCulibrarygetunifiedfunction = 383,
    /// cuKernelGetAttribute
    FuncCukernelgetattribute = 384,
    /// cuKernelSetAttribute
    FuncCukernelsetattribute = 385,
    /// cuKernelSetCacheConfig
    FuncCukernelsetcacheconfig = 386,
    /// cuKernelGetName
    FuncCukernelgetname = 387,
    /// cuKernelGetParamInfo
    FuncCukernelgetparaminfo = 388,
    /// cuMemGetInfo_v2
    FuncCumemgetinfoV2 = 389,
    /// cuMemAlloc_v2
    FuncCumemallocV2 = 390,
    /// cuMemAllocPitch_v2
    FuncCumemallocpitchV2 = 391,
    /// cuMemFree_v2
    FuncCumemfreeV2 = 392,
    /// cuMemGetAddressRange_v2
    FuncCumemgetaddressrangeV2 = 393,
    /// cuMemAllocHost_v2
    FuncCumemallochostV2 = 394,
    /// cuMemFreeHost
    FuncCumemfreehost = 395,
    /// cuMemHostAlloc
    FuncCumemhostalloc = 396,
    /// cuMemHostGetDevicePointer_v2
    FuncCumemhostgetdevicepointerV2 = 397,
    /// cuMemHostGetFlags
    FuncCumemhostgetflags = 398,
    /// cuMemAllocManaged
    FuncCumemallocmanaged = 399,
    /// cuDeviceRegisterAsyncNotification
    FuncCudeviceregisterasyncnotification = 400,
    /// cuDeviceUnregisterAsyncNotification
    FuncCudeviceunregisterasyncnotification = 401,
    /// cuDeviceGetByPCIBusId
    FuncCudevicegetbypcibusid = 402,
    /// cuDeviceGetPCIBusId
    FuncCudevicegetpcibusid = 403,
    /// cuIpcGetEventHandle
    FuncCuipcgeteventhandle = 404,
    /// cuIpcOpenEventHandle
    FuncCuipcopeneventhandle = 405,
    /// cuIpcGetMemHandle
    FuncCuipcgetmemhandle = 406,
    /// cuIpcOpenMemHandle_v2
    FuncCuipcopenmemhandleV2 = 407,
    /// cuIpcCloseMemHandle
    FuncCuipcclosememhandle = 408,
    /// cuMemHostRegister_v2
    FuncCumemhostregisterV2 = 409,
    /// cuMemHostUnregister
    FuncCumemhostunregister = 410,
    /// cuMemcpy
    FuncCumemcpy = 411,
    /// cuMemcpyPeer
    FuncCumemcpypeer = 412,
    /// cuMemcpyHtoD_v2
    FuncCumemcpyhtodV2 = 413,
    /// cuMemcpyDtoH_v2
    FuncCumemcpydtohV2 = 414,
    /// cuMemcpyDtoD_v2
    FuncCumemcpydtodV2 = 415,
    /// cuMemcpyDtoA_v2
    FuncCumemcpydtoaV2 = 416,
    /// cuMemcpyAtoD_v2
    FuncCumemcpyatodV2 = 417,
    /// cuMemcpyHtoA_v2
    FuncCumemcpyhtoaV2 = 418,
    /// cuMemcpyAtoH_v2
    FuncCumemcpyatohV2 = 419,
    /// cuMemcpyAtoA_v2
    FuncCumemcpyatoaV2 = 420,
    /// cuMemcpy2D_v2
    FuncCumemcpy2dV2 = 421,
    /// cuMemcpy2DUnaligned_v2
    FuncCumemcpy2dunalignedV2 = 422,
    /// cuMemcpy3D_v2
    FuncCumemcpy3dV2 = 423,
    /// cuMemcpy3DPeer
    FuncCumemcpy3dpeer = 424,
    /// cuMemcpyAsync
    FuncCumemcpyasync = 425,
    /// cuMemcpyPeerAsync
    FuncCumemcpypeerasync = 426,
    /// cuMemcpyHtoDAsync_v2
    FuncCumemcpyhtodasyncV2 = 427,
    /// cuMemcpyDtoHAsync_v2
    FuncCumemcpydtohasyncV2 = 428,
    /// cuMemcpyDtoDAsync_v2
    FuncCumemcpydtodasyncV2 = 429,
    /// cuMemcpyHtoAAsync_v2
    FuncCumemcpyhtoaasyncV2 = 430,
    /// cuMemcpyAtoHAsync_v2
    FuncCumemcpyatohasyncV2 = 431,
    /// cuMemcpy2DAsync_v2
    FuncCumemcpy2dasyncV2 = 432,
    /// cuMemcpy3DAsync_v2
    FuncCumemcpy3dasyncV2 = 433,
    /// cuMemcpy3DPeerAsync
    FuncCumemcpy3dpeerasync = 434,
    /// cuMemsetD8_v2
    FuncCumemsetd8V2 = 435,
    /// cuMemsetD16_v2
    FuncCumemsetd16V2 = 436,
    /// cuMemsetD32_v2
    FuncCumemsetd32V2 = 437,
    /// cuMemsetD2D8_v2
    FuncCumemsetd2d8V2 = 438,
    /// cuMemsetD2D16_v2
    FuncCumemsetd2d16V2 = 439,
    /// cuMemsetD2D32_v2
    FuncCumemsetd2d32V2 = 440,
    /// cuMemsetD8Async
    FuncCumemsetd8async = 441,
    /// cuMemsetD16Async
    FuncCumemsetd16async = 442,
    /// cuMemsetD32Async
    FuncCumemsetd32async = 443,
    /// cuMemsetD2D8Async
    FuncCumemsetd2d8async = 444,
    /// cuMemsetD2D16Async
    FuncCumemsetd2d16async = 445,
    /// cuMemsetD2D32Async
    FuncCumemsetd2d32async = 446,
    /// cuArrayCreate_v2
    FuncCuarraycreateV2 = 447,
    /// cuArrayGetDescriptor_v2
    FuncCuarraygetdescriptorV2 = 448,
    /// cuArrayGetSparseProperties
    FuncCuarraygetsparseproperties = 449,
    /// cuMipmappedArrayGetSparseProperties
    FuncCumipmappedarraygetsparseproperties = 450,
    /// cuArrayGetMemoryRequirements
    FuncCuarraygetmemoryrequirements = 451,
    /// cuMipmappedArrayGetMemoryRequirements
    FuncCumipmappedarraygetmemoryrequirements = 452,
    /// cuArrayGetPlane
    FuncCuarraygetplane = 453,
    /// cuArrayDestroy
    FuncCuarraydestroy = 454,
    /// cuArray3DCreate_v2
    FuncCuarray3dcreateV2 = 455,
    /// cuArray3DGetDescriptor_v2
    FuncCuarray3dgetdescriptorV2 = 456,
    /// cuMipmappedArrayCreate
    FuncCumipmappedarraycreate = 457,
    /// cuMipmappedArrayGetLevel
    FuncCumipmappedarraygetlevel = 458,
    /// cuMipmappedArrayDestroy
    FuncCumipmappedarraydestroy = 459,
    /// cuMemGetHandleForAddressRange
    FuncCumemgethandleforaddressrange = 460,
    /// cuMemAddressReserve
    FuncCumemaddressreserve = 461,
    /// cuMemAddressFree
    FuncCumemaddressfree = 462,
    /// cuMemCreate
    FuncCumemcreate = 463,
    /// cuMemRelease
    FuncCumemrelease = 464,
    /// cuMemMap
    FuncCumemmap = 465,
    /// cuMemMapArrayAsync
    FuncCumemmaparrayasync = 466,
    /// cuMemUnmap
    FuncCumemunmap = 467,
    /// cuMemSetAccess
    FuncCumemsetaccess = 468,
    /// cuMemGetAccess
    FuncCumemgetaccess = 469,
    /// cuMemExportToShareableHandle
    FuncCumemexporttoshareablehandle = 470,
    /// cuMemImportFromShareableHandle
    FuncCumemimportfromshareablehandle = 471,
    /// cuMemGetAllocationGranularity
    FuncCumemgetallocationgranularity = 472,
    /// cuMemGetAllocationPropertiesFromHandle
    FuncCumemgetallocationpropertiesfromhandle = 473,
    /// cuMemRetainAllocationHandle
    FuncCumemretainallocationhandle = 474,
    /// cuMemFreeAsync
    FuncCumemfreeasync = 475,
    /// cuMemAllocAsync
    FuncCumemallocasync = 476,
    /// cuMemPoolTrimTo
    FuncCumempooltrimto = 477,
    /// cuMemPoolSetAttribute
    FuncCumempoolsetattribute = 478,
    /// cuMemPoolGetAttribute
    FuncCumempoolgetattribute = 479,
    /// cuMemPoolSetAccess
    FuncCumempoolsetaccess = 480,
    /// cuMemPoolGetAccess
    FuncCumempoolgetaccess = 481,
    /// cuMemPoolCreate
    FuncCumempoolcreate = 482,
    /// cuMemPoolDestroy
    FuncCumempooldestroy = 483,
    /// cuMemAllocFromPoolAsync
    FuncCumemallocfrompoolasync = 484,
    /// cuMemPoolExportToShareableHandle
    FuncCumempoolexporttoshareablehandle = 485,
    /// cuMemPoolImportFromShareableHandle
    FuncCumempoolimportfromshareablehandle = 486,
    /// cuMemPoolExportPointer
    FuncCumempoolexportpointer = 487,
    /// cuMemPoolImportPointer
    FuncCumempoolimportpointer = 488,
    /// cuMulticastCreate
    FuncCumulticastcreate = 489,
    /// cuMulticastAddDevice
    FuncCumulticastadddevice = 490,
    /// cuMulticastBindMem
    FuncCumulticastbindmem = 491,
    /// cuMulticastBindAddr
    FuncCumulticastbindaddr = 492,
    /// cuMulticastUnbind
    FuncCumulticastunbind = 493,
    /// cuMulticastGetGranularity
    FuncCumulticastgetgranularity = 494,
    /// cuPointerGetAttribute
    FuncCupointergetattribute = 495,
    /// cuMemPrefetchAsync
    FuncCumemprefetchasync = 496,
    /// cuMemPrefetchAsync_v2
    FuncCumemprefetchasyncV2 = 497,
    /// cuMemAdvise
    FuncCumemadvise = 498,
    /// cuMemAdvise_v2
    FuncCumemadviseV2 = 499,
    /// cuMemRangeGetAttribute
    FuncCumemrangegetattribute = 500,
    /// cuMemRangeGetAttributes
    FuncCumemrangegetattributes = 501,
    /// cuPointerSetAttribute
    FuncCupointersetattribute = 502,
    /// cuPointerGetAttributes
    FuncCupointergetattributes = 503,
    /// cuStreamCreate
    FuncCustreamcreate = 504,
    /// cuStreamCreateWithPriority
    FuncCustreamcreatewithpriority = 505,
    /// cuStreamGetPriority
    FuncCustreamgetpriority = 506,
    /// cuStreamGetFlags
    FuncCustreamgetflags = 507,
    /// cuStreamGetId
    FuncCustreamgetid = 508,
    /// cuStreamGetCtx
    FuncCustreamgetctx = 509,
    /// cuStreamGetCtx_v2
    FuncCustreamgetctxV2 = 510,
    /// cuStreamWaitEvent
    FuncCustreamwaitevent = 511,
    /// cuStreamAddCallback
    FuncCustreamaddcallback = 512,
    /// cuStreamBeginCapture_v2
    FuncCustreambegincaptureV2 = 513,
    /// cuStreamBeginCaptureToGraph
    FuncCustreambegincapturetograph = 514,
    /// cuThreadExchangeStreamCaptureMode
    FuncCuthreadexchangestreamcapturemode = 515,
    /// cuStreamEndCapture
    FuncCustreamendcapture = 516,
    /// cuStreamIsCapturing
    FuncCustreamiscapturing = 517,
    /// cuStreamGetCaptureInfo_v2
    FuncCustreamgetcaptureinfoV2 = 518,
    /// cuStreamGetCaptureInfo_v3
    FuncCustreamgetcaptureinfoV3 = 519,
    /// cuStreamUpdateCaptureDependencies
    FuncCustreamupdatecapturedependencies = 520,
    /// cuStreamUpdateCaptureDependencies_v2
    FuncCustreamupdatecapturedependenciesV2 = 521,
    /// cuStreamAttachMemAsync
    FuncCustreamattachmemasync = 522,
    /// cuStreamQuery
    FuncCustreamquery = 523,
    /// cuStreamSynchronize
    FuncCustreamsynchronize = 524,
    /// cuStreamDestroy_v2
    FuncCustreamdestroyV2 = 525,
    /// cuStreamCopyAttributes
    FuncCustreamcopyattributes = 526,
    /// cuStreamGetAttribute
    FuncCustreamgetattribute = 527,
    /// cuStreamSetAttribute
    FuncCustreamsetattribute = 528,
    /// cuEventCreate
    FuncCueventcreate = 529,
    /// cuEventRecord
    FuncCueventrecord = 530,
    /// cuEventRecordWithFlags
    FuncCueventrecordwithflags = 531,
    /// cuEventQuery
    FuncCueventquery = 532,
    /// cuEventSynchronize
    FuncCueventsynchronize = 533,
    /// cuEventDestroy_v2
    FuncCueventdestroyV2 = 534,
    /// cuEventElapsedTime
    FuncCueventelapsedtime = 535,
    /// cuImportExternalMemory
    FuncCuimportexternalmemory = 536,
    /// cuExternalMemoryGetMappedBuffer
    FuncCuexternalmemorygetmappedbuffer = 537,
    /// cuExternalMemoryGetMappedMipmappedArray
    FuncCuexternalmemorygetmappedmipmappedarray = 538,
    /// cuDestroyExternalMemory
    FuncCudestroyexternalmemory = 539,
    /// cuImportExternalSemaphore
    FuncCuimportexternalsemaphore = 540,
    /// cuSignalExternalSemaphoresAsync
    FuncCusignalexternalsemaphoresasync = 541,
    /// cuWaitExternalSemaphoresAsync
    FuncCuwaitexternalsemaphoresasync = 542,
    /// cuDestroyExternalSemaphore
    FuncCudestroyexternalsemaphore = 543,
    /// cuStreamWaitValue32_v2
    FuncCustreamwaitvalue32V2 = 544,
    /// cuStreamWaitValue64_v2
    FuncCustreamwaitvalue64V2 = 545,
    /// cuStreamWriteValue32_v2
    FuncCustreamwritevalue32V2 = 546,
    /// cuStreamWriteValue64_v2
    FuncCustreamwritevalue64V2 = 547,
    /// cuStreamBatchMemOp_v2
    FuncCustreambatchmemopV2 = 548,
    /// cuFuncGetAttribute
    FuncCufuncgetattribute = 549,
    /// cuFuncSetAttribute
    FuncCufuncsetattribute = 550,
    /// cuFuncSetCacheConfig
    FuncCufuncsetcacheconfig = 551,
    /// cuFuncGetModule
    FuncCufuncgetmodule = 552,
    /// cuFuncGetName
    FuncCufuncgetname = 553,
    /// cuFuncGetParamInfo
    FuncCufuncgetparaminfo = 554,
    /// cuFuncIsLoaded
    FuncCufuncisloaded = 555,
    /// cuFuncLoad
    FuncCufuncload = 556,
    /// cuLaunchKernel
    FuncCulaunchkernel = 557,
    /// cuLaunchKernelEx
    FuncCulaunchkernelex = 558,
    /// cuLaunchCooperativeKernel
    FuncCulaunchcooperativekernel = 559,
    /// cuLaunchCooperativeKernelMultiDevice
    FuncCulaunchcooperativekernelmultidevice = 560,
    /// cuLaunchHostFunc
    FuncCulaunchhostfunc = 561,
    /// cuFuncSetBlockShape
    FuncCufuncsetblockshape = 562,
    /// cuFuncSetSharedSize
    FuncCufuncsetsharedsize = 563,
    /// cuParamSetSize
    FuncCuparamsetsize = 564,
    /// cuParamSeti
    FuncCuparamseti = 565,
    /// cuParamSetf
    FuncCuparamsetf = 566,
    /// cuParamSetv
    FuncCuparamsetv = 567,
    /// cuLaunch
    FuncCulaunch = 568,
    /// cuLaunchGrid
    FuncCulaunchgrid = 569,
    /// cuLaunchGridAsync
    FuncCulaunchgridasync = 570,
    /// cuParamSetTexRef
    FuncCuparamsettexref = 571,
    /// cuFuncSetSharedMemConfig
    FuncCufuncsetsharedmemconfig = 572,
    /// cuGraphCreate
    FuncCugraphcreate = 573,
    /// cuGraphAddKernelNode_v2
    FuncCugraphaddkernelnodeV2 = 574,
    /// cuGraphKernelNodeGetParams_v2
    FuncCugraphkernelnodegetparamsV2 = 575,
    /// cuGraphKernelNodeSetParams_v2
    FuncCugraphkernelnodesetparamsV2 = 576,
    /// cuGraphAddMemcpyNode
    FuncCugraphaddmemcpynode = 577,
    /// cuGraphMemcpyNodeGetParams
    FuncCugraphmemcpynodegetparams = 578,
    /// cuGraphMemcpyNodeSetParams
    FuncCugraphmemcpynodesetparams = 579,
    /// cuGraphAddMemsetNode
    FuncCugraphaddmemsetnode = 580,
    /// cuGraphMemsetNodeGetParams
    FuncCugraphmemsetnodegetparams = 581,
    /// cuGraphMemsetNodeSetParams
    FuncCugraphmemsetnodesetparams = 582,
    /// cuGraphAddHostNode
    FuncCugraphaddhostnode = 583,
    /// cuGraphHostNodeGetParams
    FuncCugraphhostnodegetparams = 584,
    /// cuGraphHostNodeSetParams
    FuncCugraphhostnodesetparams = 585,
    /// cuGraphAddChildGraphNode
    FuncCugraphaddchildgraphnode = 586,
    /// cuGraphChildGraphNodeGetGraph
    FuncCugraphchildgraphnodegetgraph = 587,
    /// cuGraphAddEmptyNode
    FuncCugraphaddemptynode = 588,
    /// cuGraphAddEventRecordNode
    FuncCugraphaddeventrecordnode = 589,
    /// cuGraphEventRecordNodeGetEvent
    FuncCugrapheventrecordnodegetevent = 590,
    /// cuGraphEventRecordNodeSetEvent
    FuncCugrapheventrecordnodesetevent = 591,
    /// cuGraphAddEventWaitNode
    FuncCugraphaddeventwaitnode = 592,
    /// cuGraphEventWaitNodeGetEvent
    FuncCugrapheventwaitnodegetevent = 593,
    /// cuGraphEventWaitNodeSetEvent
    FuncCugrapheventwaitnodesetevent = 594,
    /// cuGraphAddExternalSemaphoresSignalNode
    FuncCugraphaddexternalsemaphoressignalnode = 595,
    /// cuGraphExternalSemaphoresSignalNodeGetParams
    FuncCugraphexternalsemaphoressignalnodegetparams = 596,
    /// cuGraphExternalSemaphoresSignalNodeSetParams
    FuncCugraphexternalsemaphoressignalnodesetparams = 597,
    /// cuGraphAddExternalSemaphoresWaitNode
    FuncCugraphaddexternalsemaphoreswaitnode = 598,
    /// cuGraphExternalSemaphoresWaitNodeGetParams
    FuncCugraphexternalsemaphoreswaitnodegetparams = 599,
    /// cuGraphExternalSemaphoresWaitNodeSetParams
    FuncCugraphexternalsemaphoreswaitnodesetparams = 600,
    /// cuGraphAddBatchMemOpNode
    FuncCugraphaddbatchmemopnode = 601,
    /// cuGraphBatchMemOpNodeGetParams
    FuncCugraphbatchmemopnodegetparams = 602,
    /// cuGraphBatchMemOpNodeSetParams
    FuncCugraphbatchmemopnodesetparams = 603,
    /// cuGraphExecBatchMemOpNodeSetParams
    FuncCugraphexecbatchmemopnodesetparams = 604,
    /// cuGraphAddMemAllocNode
    FuncCugraphaddmemallocnode = 605,
    /// cuGraphMemAllocNodeGetParams
    FuncCugraphmemallocnodegetparams = 606,
    /// cuGraphAddMemFreeNode
    FuncCugraphaddmemfreenode = 607,
    /// cuGraphMemFreeNodeGetParams
    FuncCugraphmemfreenodegetparams = 608,
    /// cuDeviceGraphMemTrim
    FuncCudevicegraphmemtrim = 609,
    /// cuDeviceGetGraphMemAttribute
    FuncCudevicegetgraphmemattribute = 610,
    /// cuDeviceSetGraphMemAttribute
    FuncCudevicesetgraphmemattribute = 611,
    /// cuGraphClone
    FuncCugraphclone = 612,
    /// cuGraphNodeFindInClone
    FuncCugraphnodefindinclone = 613,
    /// cuGraphNodeGetType
    FuncCugraphnodegettype = 614,
    /// cuGraphGetNodes
    FuncCugraphgetnodes = 615,
    /// cuGraphGetRootNodes
    FuncCugraphgetrootnodes = 616,
    /// cuGraphGetEdges
    FuncCugraphgetedges = 617,
    /// cuGraphGetEdges_v2
    FuncCugraphgetedgesV2 = 618,
    /// cuGraphNodeGetDependencies
    FuncCugraphnodegetdependencies = 619,
    /// cuGraphNodeGetDependencies_v2
    FuncCugraphnodegetdependenciesV2 = 620,
    /// cuGraphNodeGetDependentNodes
    FuncCugraphnodegetdependentnodes = 621,
    /// cuGraphNodeGetDependentNodes_v2
    FuncCugraphnodegetdependentnodesV2 = 622,
    /// cuGraphAddDependencies
    FuncCugraphadddependencies = 623,
    /// cuGraphAddDependencies_v2
    FuncCugraphadddependenciesV2 = 624,
    /// cuGraphRemoveDependencies
    FuncCugraphremovedependencies = 625,
    /// cuGraphRemoveDependencies_v2
    FuncCugraphremovedependenciesV2 = 626,
    /// cuGraphDestroyNode
    FuncCugraphdestroynode = 627,
    /// cuGraphInstantiateWithFlags
    FuncCugraphinstantiatewithflags = 628,
    /// cuGraphInstantiateWithParams
    FuncCugraphinstantiatewithparams = 629,
    /// cuGraphExecGetFlags
    FuncCugraphexecgetflags = 630,
    /// cuGraphExecKernelNodeSetParams_v2
    FuncCugraphexeckernelnodesetparamsV2 = 631,
    /// cuGraphExecMemcpyNodeSetParams
    FuncCugraphexecmemcpynodesetparams = 632,
    /// cuGraphExecMemsetNodeSetParams
    FuncCugraphexecmemsetnodesetparams = 633,
    /// cuGraphExecHostNodeSetParams
    FuncCugraphexechostnodesetparams = 634,
    /// cuGraphExecChildGraphNodeSetParams
    FuncCugraphexecchildgraphnodesetparams = 635,
    /// cuGraphExecEventRecordNodeSetEvent
    FuncCugraphexeceventrecordnodesetevent = 636,
    /// cuGraphExecEventWaitNodeSetEvent
    FuncCugraphexeceventwaitnodesetevent = 637,
    /// cuGraphExecExternalSemaphoresSignalNodeSetParams
    FuncCugraphexecexternalsemaphoressignalnodesetparams = 638,
    /// cuGraphExecExternalSemaphoresWaitNodeSetParams
    FuncCugraphexecexternalsemaphoreswaitnodesetparams = 639,
    /// cuGraphNodeSetEnabled
    FuncCugraphnodesetenabled = 640,
    /// cuGraphNodeGetEnabled
    FuncCugraphnodegetenabled = 641,
    /// cuGraphUpload
    FuncCugraphupload = 642,
    /// cuGraphLaunch
    FuncCugraphlaunch = 643,
    /// cuGraphExecDestroy
    FuncCugraphexecdestroy = 644,
    /// cuGraphDestroy
    FuncCugraphdestroy = 645,
    /// cuGraphExecUpdate_v2
    FuncCugraphexecupdateV2 = 646,
    /// cuGraphKernelNodeCopyAttributes
    FuncCugraphkernelnodecopyattributes = 647,
    /// cuGraphKernelNodeGetAttribute
    FuncCugraphkernelnodegetattribute = 648,
    /// cuGraphKernelNodeSetAttribute
    FuncCugraphkernelnodesetattribute = 649,
    /// cuGraphDebugDotPrint
    FuncCugraphdebugdotprint = 650,
    /// cuUserObjectCreate
    FuncCuuserobjectcreate = 651,
    /// cuUserObjectRetain
    FuncCuuserobjectretain = 652,
    /// cuUserObjectRelease
    FuncCuuserobjectrelease = 653,
    /// cuGraphRetainUserObject
    FuncCugraphretainuserobject = 654,
    /// cuGraphReleaseUserObject
    FuncCugraphreleaseuserobject = 655,
    /// cuGraphAddNode
    FuncCugraphaddnode = 656,
    /// cuGraphAddNode_v2
    FuncCugraphaddnodeV2 = 657,
    /// cuGraphNodeSetParams
    FuncCugraphnodesetparams = 658,
    /// cuGraphExecNodeSetParams
    FuncCugraphexecnodesetparams = 659,
    /// cuGraphConditionalHandleCreate
    FuncCugraphconditionalhandlecreate = 660,
    /// cuOccupancyMaxActiveBlocksPerMultiprocessor
    FuncCuoccupancymaxactiveblockspermultiprocessor = 661,
    /// cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags
    FuncCuoccupancymaxactiveblockspermultiprocessorwithflags = 662,
    /// cuOccupancyMaxPotentialBlockSize
    FuncCuoccupancymaxpotentialblocksize = 663,
    /// cuOccupancyMaxPotentialBlockSizeWithFlags
    FuncCuoccupancymaxpotentialblocksizewithflags = 664,
    /// cuOccupancyAvailableDynamicSMemPerBlock
    FuncCuoccupancyavailabledynamicsmemperblock = 665,
    /// cuOccupancyMaxPotentialClusterSize
    FuncCuoccupancymaxpotentialclustersize = 666,
    /// cuOccupancyMaxActiveClusters
    FuncCuoccupancymaxactiveclusters = 667,
    /// cuTexRefSetArray
    FuncCutexrefsetarray = 668,
    /// cuTexRefSetMipmappedArray
    FuncCutexrefsetmipmappedarray = 669,
    /// cuTexRefSetAddress_v2
    FuncCutexrefsetaddressV2 = 670,
    /// cuTexRefSetAddress2D_v3
    FuncCutexrefsetaddress2dV3 = 671,
    /// cuTexRefSetFormat
    FuncCutexrefsetformat = 672,
    /// cuTexRefSetAddressMode
    FuncCutexrefsetaddressmode = 673,
    /// cuTexRefSetFilterMode
    FuncCutexrefsetfiltermode = 674,
    /// cuTexRefSetMipmapFilterMode
    FuncCutexrefsetmipmapfiltermode = 675,
    /// cuTexRefSetMipmapLevelBias
    FuncCutexrefsetmipmaplevelbias = 676,
    /// cuTexRefSetMipmapLevelClamp
    FuncCutexrefsetmipmaplevelclamp = 677,
    /// cuTexRefSetMaxAnisotropy
    FuncCutexrefsetmaxanisotropy = 678,
    /// cuTexRefSetBorderColor
    FuncCutexrefsetbordercolor = 679,
    /// cuTexRefSetFlags
    FuncCutexrefsetflags = 680,
    /// cuTexRefGetAddress_v2
    FuncCutexrefgetaddressV2 = 681,
    /// cuTexRefGetArray
    FuncCutexrefgetarray = 682,
    /// cuTexRefGetMipmappedArray
    FuncCutexrefgetmipmappedarray = 683,
    /// cuTexRefGetAddressMode
    FuncCutexrefgetaddressmode = 684,
    /// cuTexRefGetFilterMode
    FuncCutexrefgetfiltermode = 685,
    /// cuTexRefGetFormat
    FuncCutexrefgetformat = 686,
    /// cuTexRefGetMipmapFilterMode
    FuncCutexrefgetmipmapfiltermode = 687,
    /// cuTexRefGetMipmapLevelBias
    FuncCutexrefgetmipmaplevelbias = 688,
    /// cuTexRefGetMipmapLevelClamp
    FuncCutexrefgetmipmaplevelclamp = 689,
    /// cuTexRefGetMaxAnisotropy
    FuncCutexrefgetmaxanisotropy = 690,
    /// cuTexRefGetBorderColor
    FuncCutexrefgetbordercolor = 691,
    /// cuTexRefGetFlags
    FuncCutexrefgetflags = 692,
    /// cuTexRefCreate
    FuncCutexrefcreate = 693,
    /// cuTexRefDestroy
    FuncCutexrefdestroy = 694,
    /// cuSurfRefSetArray
    FuncCusurfrefsetarray = 695,
    /// cuSurfRefGetArray
    FuncCusurfrefgetarray = 696,
    /// cuTexObjectCreate
    FuncCutexobjectcreate = 697,
    /// cuTexObjectDestroy
    FuncCutexobjectdestroy = 698,
    /// cuTexObjectGetResourceDesc
    FuncCutexobjectgetresourcedesc = 699,
    /// cuTexObjectGetTextureDesc
    FuncCutexobjectgettexturedesc = 700,
    /// cuTexObjectGetResourceViewDesc
    FuncCutexobjectgetresourceviewdesc = 701,
    /// cuSurfObjectCreate
    FuncCusurfobjectcreate = 702,
    /// cuSurfObjectDestroy
    FuncCusurfobjectdestroy = 703,
    /// cuSurfObjectGetResourceDesc
    FuncCusurfobjectgetresourcedesc = 704,
    /// cuTensorMapEncodeTiled
    FuncCutensormapencodetiled = 705,
    /// cuTensorMapEncodeIm2col
    FuncCutensormapencodeim2col = 706,
    /// cuTensorMapReplaceAddress
    FuncCutensormapreplaceaddress = 707,
    /// cuDeviceCanAccessPeer
    FuncCudevicecanaccesspeer = 708,
    /// cuCtxEnablePeerAccess
    FuncCuctxenablepeeraccess = 709,
    /// cuCtxDisablePeerAccess
    FuncCuctxdisablepeeraccess = 710,
    /// cuDeviceGetP2PAttribute
    FuncCudevicegetp2pattribute = 711,
    /// cuGraphicsUnregisterResource
    FuncCugraphicsunregisterresource = 712,
    /// cuGraphicsSubResourceGetMappedArray
    FuncCugraphicssubresourcegetmappedarray = 713,
    /// cuGraphicsResourceGetMappedMipmappedArray
    FuncCugraphicsresourcegetmappedmipmappedarray = 714,
    /// cuGraphicsResourceGetMappedPointer_v2
    FuncCugraphicsresourcegetmappedpointerV2 = 715,
    /// cuGraphicsResourceSetMapFlags_v2
    FuncCugraphicsresourcesetmapflagsV2 = 716,
    /// cuGraphicsMapResources
    FuncCugraphicsmapresources = 717,
    /// cuGraphicsUnmapResources
    FuncCugraphicsunmapresources = 718,
    /// cuGetProcAddress_v2
    FuncCugetprocaddressV2 = 719,
    /// cuCoredumpGetAttribute
    FuncCucoredumpgetattribute = 720,
    /// cuCoredumpGetAttributeGlobal
    FuncCucoredumpgetattributeglobal = 721,
    /// cuCoredumpSetAttribute
    FuncCucoredumpsetattribute = 722,
    /// cuCoredumpSetAttributeGlobal
    FuncCucoredumpsetattributeglobal = 723,
    /// cuGetExportTable
    FuncCugetexporttable = 724,
    /// cuGreenCtxCreate
    FuncCugreenctxcreate = 725,
    /// cuGreenCtxDestroy
    FuncCugreenctxdestroy = 726,
    /// cuCtxFromGreenCtx
    FuncCuctxfromgreenctx = 727,
    /// cuDeviceGetDevResource
    FuncCudevicegetdevresource = 728,
    /// cuCtxGetDevResource
    FuncCuctxgetdevresource = 729,
    /// cuGreenCtxGetDevResource
    FuncCugreenctxgetdevresource = 730,
    /// cuDevSmResourceSplitByCount
    FuncCudevsmresourcesplitbycount = 731,
    /// cuDevResourceGenerateDesc
    FuncCudevresourcegeneratedesc = 732,
    /// cuGreenCtxRecordEvent
    FuncCugreenctxrecordevent = 733,
    /// cuGreenCtxWaitEvent
    FuncCugreenctxwaitevent = 734,
    /// cuStreamGetGreenCtx
    FuncCustreamgetgreenctx = 735,
    /// cuGreenCtxStreamCreate
    FuncCugreenctxstreamcreate = 736,
    /// cublasCreate_v2
    FuncCublascreateV2 = 737,
    /// cublasDestroy_v2
    FuncCublasdestroyV2 = 738,
    /// cublasGetVersion_v2
    FuncCublasgetversionV2 = 739,
    /// cublasGetProperty
    FuncCublasgetproperty = 740,
    /// cublasGetCudartVersion
    FuncCublasgetcudartversion = 741,
    /// cublasSetWorkspace_v2
    FuncCublassetworkspaceV2 = 742,
    /// cublasSetStream_v2
    FuncCublassetstreamV2 = 743,
    /// cublasGetStream_v2
    FuncCublasgetstreamV2 = 744,
    /// cublasGetPointerMode_v2
    FuncCublasgetpointermodeV2 = 745,
    /// cublasSetPointerMode_v2
    FuncCublassetpointermodeV2 = 746,
    /// cublasGetAtomicsMode
    FuncCublasgetatomicsmode = 747,
    /// cublasSetAtomicsMode
    FuncCublassetatomicsmode = 748,
    /// cublasGetMathMode
    FuncCublasgetmathmode = 749,
    /// cublasSetMathMode
    FuncCublassetmathmode = 750,
    /// cublasGetSmCountTarget
    FuncCublasgetsmcounttarget = 751,
    /// cublasSetSmCountTarget
    FuncCublassetsmcounttarget = 752,
    /// cublasGetStatusName
    FuncCublasgetstatusname = 753,
    /// cublasGetStatusString
    FuncCublasgetstatusstring = 754,
    /// cublasLoggerConfigure
    FuncCublasloggerconfigure = 755,
    /// cublasSetLoggerCallback
    FuncCublassetloggercallback = 756,
    /// cublasGetLoggerCallback
    FuncCublasgetloggercallback = 757,
    /// cublasSetVector
    FuncCublassetvector = 758,
    /// cublasSetVector_64
    FuncCublassetvector64 = 759,
    /// cublasGetVector
    FuncCublasgetvector = 760,
    /// cublasGetVector_64
    FuncCublasgetvector64 = 761,
    /// cublasSetMatrix
    FuncCublassetmatrix = 762,
    /// cublasSetMatrix_64
    FuncCublassetmatrix64 = 763,
    /// cublasGetMatrix
    FuncCublasgetmatrix = 764,
    /// cublasGetMatrix_64
    FuncCublasgetmatrix64 = 765,
    /// cublasSetVectorAsync
    FuncCublassetvectorasync = 766,
    /// cublasSetVectorAsync_64
    FuncCublassetvectorasync64 = 767,
    /// cublasGetVectorAsync
    FuncCublasgetvectorasync = 768,
    /// cublasGetVectorAsync_64
    FuncCublasgetvectorasync64 = 769,
    /// cublasSetMatrixAsync
    FuncCublassetmatrixasync = 770,
    /// cublasSetMatrixAsync_64
    FuncCublassetmatrixasync64 = 771,
    /// cublasGetMatrixAsync
    FuncCublasgetmatrixasync = 772,
    /// cublasGetMatrixAsync_64
    FuncCublasgetmatrixasync64 = 773,
    /// cublasXerbla
    FuncCublasxerbla = 774,
    /// cublasNrm2Ex
    FuncCublasnrm2ex = 775,
    /// cublasNrm2Ex_64
    FuncCublasnrm2ex64 = 776,
    /// cublasSnrm2_v2
    FuncCublassnrm2V2 = 777,
    /// cublasSnrm2_v2_64
    FuncCublassnrm2V264 = 778,
    /// cublasDnrm2_v2
    FuncCublasdnrm2V2 = 779,
    /// cublasDnrm2_v2_64
    FuncCublasdnrm2V264 = 780,
    /// cublasScnrm2_v2
    FuncCublasscnrm2V2 = 781,
    /// cublasScnrm2_v2_64
    FuncCublasscnrm2V264 = 782,
    /// cublasDznrm2_v2
    FuncCublasdznrm2V2 = 783,
    /// cublasDznrm2_v2_64
    FuncCublasdznrm2V264 = 784,
    /// cublasDotEx
    FuncCublasdotex = 785,
    /// cublasDotEx_64
    FuncCublasdotex64 = 786,
    /// cublasDotcEx
    FuncCublasdotcex = 787,
    /// cublasDotcEx_64
    FuncCublasdotcex64 = 788,
    /// cublasSdot_v2
    FuncCublassdotV2 = 789,
    /// cublasSdot_v2_64
    FuncCublassdotV264 = 790,
    /// cublasDdot_v2
    FuncCublasddotV2 = 791,
    /// cublasDdot_v2_64
    FuncCublasddotV264 = 792,
    /// cublasCdotu_v2
    FuncCublascdotuV2 = 793,
    /// cublasCdotu_v2_64
    FuncCublascdotuV264 = 794,
    /// cublasCdotc_v2
    FuncCublascdotcV2 = 795,
    /// cublasCdotc_v2_64
    FuncCublascdotcV264 = 796,
    /// cublasZdotu_v2
    FuncCublaszdotuV2 = 797,
    /// cublasZdotu_v2_64
    FuncCublaszdotuV264 = 798,
    /// cublasZdotc_v2
    FuncCublaszdotcV2 = 799,
    /// cublasZdotc_v2_64
    FuncCublaszdotcV264 = 800,
    /// cublasScalEx
    FuncCublasscalex = 801,
    /// cublasScalEx_64
    FuncCublasscalex64 = 802,
    /// cublasSscal_v2
    FuncCublassscalV2 = 803,
    /// cublasSscal_v2_64
    FuncCublassscalV264 = 804,
    /// cublasDscal_v2
    FuncCublasdscalV2 = 805,
    /// cublasDscal_v2_64
    FuncCublasdscalV264 = 806,
    /// cublasCscal_v2
    FuncCublascscalV2 = 807,
    /// cublasCscal_v2_64
    FuncCublascscalV264 = 808,
    /// cublasCsscal_v2
    FuncCublascsscalV2 = 809,
    /// cublasCsscal_v2_64
    FuncCublascsscalV264 = 810,
    /// cublasZscal_v2
    FuncCublaszscalV2 = 811,
    /// cublasZscal_v2_64
    FuncCublaszscalV264 = 812,
    /// cublasZdscal_v2
    FuncCublaszdscalV2 = 813,
    /// cublasZdscal_v2_64
    FuncCublaszdscalV264 = 814,
    /// cublasAxpyEx
    FuncCublasaxpyex = 815,
    /// cublasAxpyEx_64
    FuncCublasaxpyex64 = 816,
    /// cublasSaxpy_v2
    FuncCublassaxpyV2 = 817,
    /// cublasSaxpy_v2_64
    FuncCublassaxpyV264 = 818,
    /// cublasDaxpy_v2
    FuncCublasdaxpyV2 = 819,
    /// cublasDaxpy_v2_64
    FuncCublasdaxpyV264 = 820,
    /// cublasCaxpy_v2
    FuncCublascaxpyV2 = 821,
    /// cublasCaxpy_v2_64
    FuncCublascaxpyV264 = 822,
    /// cublasZaxpy_v2
    FuncCublaszaxpyV2 = 823,
    /// cublasZaxpy_v2_64
    FuncCublaszaxpyV264 = 824,
    /// cublasCopyEx
    FuncCublascopyex = 825,
    /// cublasCopyEx_64
    FuncCublascopyex64 = 826,
    /// cublasScopy_v2
    FuncCublasscopyV2 = 827,
    /// cublasScopy_v2_64
    FuncCublasscopyV264 = 828,
    /// cublasDcopy_v2
    FuncCublasdcopyV2 = 829,
    /// cublasDcopy_v2_64
    FuncCublasdcopyV264 = 830,
    /// cublasCcopy_v2
    FuncCublasccopyV2 = 831,
    /// cublasCcopy_v2_64
    FuncCublasccopyV264 = 832,
    /// cublasZcopy_v2
    FuncCublaszcopyV2 = 833,
    /// cublasZcopy_v2_64
    FuncCublaszcopyV264 = 834,
    /// cublasSswap_v2
    FuncCublassswapV2 = 835,
    /// cublasSswap_v2_64
    FuncCublassswapV264 = 836,
    /// cublasDswap_v2
    FuncCublasdswapV2 = 837,
    /// cublasDswap_v2_64
    FuncCublasdswapV264 = 838,
    /// cublasCswap_v2
    FuncCublascswapV2 = 839,
    /// cublasCswap_v2_64
    FuncCublascswapV264 = 840,
    /// cublasZswap_v2
    FuncCublaszswapV2 = 841,
    /// cublasZswap_v2_64
    FuncCublaszswapV264 = 842,
    /// cublasSwapEx
    FuncCublasswapex = 843,
    /// cublasSwapEx_64
    FuncCublasswapex64 = 844,
    /// cublasIsamax_v2
    FuncCublasisamaxV2 = 845,
    /// cublasIsamax_v2_64
    FuncCublasisamaxV264 = 846,
    /// cublasIdamax_v2
    FuncCublasidamaxV2 = 847,
    /// cublasIdamax_v2_64
    FuncCublasidamaxV264 = 848,
    /// cublasIcamax_v2
    FuncCublasicamaxV2 = 849,
    /// cublasIcamax_v2_64
    FuncCublasicamaxV264 = 850,
    /// cublasIzamax_v2
    FuncCublasizamaxV2 = 851,
    /// cublasIzamax_v2_64
    FuncCublasizamaxV264 = 852,
    /// cublasIamaxEx
    FuncCublasiamaxex = 853,
    /// cublasIamaxEx_64
    FuncCublasiamaxex64 = 854,
    /// cublasIsamin_v2
    FuncCublasisaminV2 = 855,
    /// cublasIsamin_v2_64
    FuncCublasisaminV264 = 856,
    /// cublasIdamin_v2
    FuncCublasidaminV2 = 857,
    /// cublasIdamin_v2_64
    FuncCublasidaminV264 = 858,
    /// cublasIcamin_v2
    FuncCublasicaminV2 = 859,
    /// cublasIcamin_v2_64
    FuncCublasicaminV264 = 860,
    /// cublasIzamin_v2
    FuncCublasizaminV2 = 861,
    /// cublasIzamin_v2_64
    FuncCublasizaminV264 = 862,
    /// cublasIaminEx
    FuncCublasiaminex = 863,
    /// cublasIaminEx_64
    FuncCublasiaminex64 = 864,
    /// cublasAsumEx
    FuncCublasasumex = 865,
    /// cublasAsumEx_64
    FuncCublasasumex64 = 866,
    /// cublasSasum_v2
    FuncCublassasumV2 = 867,
    /// cublasSasum_v2_64
    FuncCublassasumV264 = 868,
    /// cublasDasum_v2
    FuncCublasdasumV2 = 869,
    /// cublasDasum_v2_64
    FuncCublasdasumV264 = 870,
    /// cublasScasum_v2
    FuncCublasscasumV2 = 871,
    /// cublasScasum_v2_64
    FuncCublasscasumV264 = 872,
    /// cublasDzasum_v2
    FuncCublasdzasumV2 = 873,
    /// cublasDzasum_v2_64
    FuncCublasdzasumV264 = 874,
    /// cublasSrot_v2
    FuncCublassrotV2 = 875,
    /// cublasSrot_v2_64
    FuncCublassrotV264 = 876,
    /// cublasDrot_v2
    FuncCublasdrotV2 = 877,
    /// cublasDrot_v2_64
    FuncCublasdrotV264 = 878,
    /// cublasCrot_v2
    FuncCublascrotV2 = 879,
    /// cublasCrot_v2_64
    FuncCublascrotV264 = 880,
    /// cublasCsrot_v2
    FuncCublascsrotV2 = 881,
    /// cublasCsrot_v2_64
    FuncCublascsrotV264 = 882,
    /// cublasZrot_v2
    FuncCublaszrotV2 = 883,
    /// cublasZrot_v2_64
    FuncCublaszrotV264 = 884,
    /// cublasZdrot_v2
    FuncCublaszdrotV2 = 885,
    /// cublasZdrot_v2_64
    FuncCublaszdrotV264 = 886,
    /// cublasRotEx
    FuncCublasrotex = 887,
    /// cublasRotEx_64
    FuncCublasrotex64 = 888,
    /// cublasSrotg_v2
    FuncCublassrotgV2 = 889,
    /// cublasDrotg_v2
    FuncCublasdrotgV2 = 890,
    /// cublasCrotg_v2
    FuncCublascrotgV2 = 891,
    /// cublasZrotg_v2
    FuncCublaszrotgV2 = 892,
    /// cublasRotgEx
    FuncCublasrotgex = 893,
    /// cublasSrotm_v2
    FuncCublassrotmV2 = 894,
    /// cublasSrotm_v2_64
    FuncCublassrotmV264 = 895,
    /// cublasDrotm_v2
    FuncCublasdrotmV2 = 896,
    /// cublasDrotm_v2_64
    FuncCublasdrotmV264 = 897,
    /// cublasRotmEx
    FuncCublasrotmex = 898,
    /// cublasRotmEx_64
    FuncCublasrotmex64 = 899,
    /// cublasSrotmg_v2
    FuncCublassrotmgV2 = 900,
    /// cublasDrotmg_v2
    FuncCublasdrotmgV2 = 901,
    /// cublasRotmgEx
    FuncCublasrotmgex = 902,
    /// cublasSgemv_v2
    FuncCublassgemvV2 = 903,
    /// cublasSgemv_v2_64
    FuncCublassgemvV264 = 904,
    /// cublasDgemv_v2
    FuncCublasdgemvV2 = 905,
    /// cublasDgemv_v2_64
    FuncCublasdgemvV264 = 906,
    /// cublasCgemv_v2
    FuncCublascgemvV2 = 907,
    /// cublasCgemv_v2_64
    FuncCublascgemvV264 = 908,
    /// cublasZgemv_v2
    FuncCublaszgemvV2 = 909,
    /// cublasZgemv_v2_64
    FuncCublaszgemvV264 = 910,
    /// cublasSgbmv_v2
    FuncCublassgbmvV2 = 911,
    /// cublasSgbmv_v2_64
    FuncCublassgbmvV264 = 912,
    /// cublasDgbmv_v2
    FuncCublasdgbmvV2 = 913,
    /// cublasDgbmv_v2_64
    FuncCublasdgbmvV264 = 914,
    /// cublasCgbmv_v2
    FuncCublascgbmvV2 = 915,
    /// cublasCgbmv_v2_64
    FuncCublascgbmvV264 = 916,
    /// cublasZgbmv_v2
    FuncCublaszgbmvV2 = 917,
    /// cublasZgbmv_v2_64
    FuncCublaszgbmvV264 = 918,
    /// cublasStrmv_v2
    FuncCublasstrmvV2 = 919,
    /// cublasStrmv_v2_64
    FuncCublasstrmvV264 = 920,
    /// cublasDtrmv_v2
    FuncCublasdtrmvV2 = 921,
    /// cublasDtrmv_v2_64
    FuncCublasdtrmvV264 = 922,
    /// cublasCtrmv_v2
    FuncCublasctrmvV2 = 923,
    /// cublasCtrmv_v2_64
    FuncCublasctrmvV264 = 924,
    /// cublasZtrmv_v2
    FuncCublasztrmvV2 = 925,
    /// cublasZtrmv_v2_64
    FuncCublasztrmvV264 = 926,
    /// cublasStbmv_v2
    FuncCublasstbmvV2 = 927,
    /// cublasStbmv_v2_64
    FuncCublasstbmvV264 = 928,
    /// cublasDtbmv_v2
    FuncCublasdtbmvV2 = 929,
    /// cublasDtbmv_v2_64
    FuncCublasdtbmvV264 = 930,
    /// cublasCtbmv_v2
    FuncCublasctbmvV2 = 931,
    /// cublasCtbmv_v2_64
    FuncCublasctbmvV264 = 932,
    /// cublasZtbmv_v2
    FuncCublasztbmvV2 = 933,
    /// cublasZtbmv_v2_64
    FuncCublasztbmvV264 = 934,
    /// cublasStpmv_v2
    FuncCublasstpmvV2 = 935,
    /// cublasStpmv_v2_64
    FuncCublasstpmvV264 = 936,
    /// cublasDtpmv_v2
    FuncCublasdtpmvV2 = 937,
    /// cublasDtpmv_v2_64
    FuncCublasdtpmvV264 = 938,
    /// cublasCtpmv_v2
    FuncCublasctpmvV2 = 939,
    /// cublasCtpmv_v2_64
    FuncCublasctpmvV264 = 940,
    /// cublasZtpmv_v2
    FuncCublasztpmvV2 = 941,
    /// cublasZtpmv_v2_64
    FuncCublasztpmvV264 = 942,
    /// cublasStrsv_v2
    FuncCublasstrsvV2 = 943,
    /// cublasStrsv_v2_64
    FuncCublasstrsvV264 = 944,
    /// cublasDtrsv_v2
    FuncCublasdtrsvV2 = 945,
    /// cublasDtrsv_v2_64
    FuncCublasdtrsvV264 = 946,
    /// cublasCtrsv_v2
    FuncCublasctrsvV2 = 947,
    /// cublasCtrsv_v2_64
    FuncCublasctrsvV264 = 948,
    /// cublasZtrsv_v2
    FuncCublasztrsvV2 = 949,
    /// cublasZtrsv_v2_64
    FuncCublasztrsvV264 = 950,
    /// cublasStpsv_v2
    FuncCublasstpsvV2 = 951,
    /// cublasStpsv_v2_64
    FuncCublasstpsvV264 = 952,
    /// cublasDtpsv_v2
    FuncCublasdtpsvV2 = 953,
    /// cublasDtpsv_v2_64
    FuncCublasdtpsvV264 = 954,
    /// cublasCtpsv_v2
    FuncCublasctpsvV2 = 955,
    /// cublasCtpsv_v2_64
    FuncCublasctpsvV264 = 956,
    /// cublasZtpsv_v2
    FuncCublasztpsvV2 = 957,
    /// cublasZtpsv_v2_64
    FuncCublasztpsvV264 = 958,
    /// cublasStbsv_v2
    FuncCublasstbsvV2 = 959,
    /// cublasStbsv_v2_64
    FuncCublasstbsvV264 = 960,
    /// cublasDtbsv_v2
    FuncCublasdtbsvV2 = 961,
    /// cublasDtbsv_v2_64
    FuncCublasdtbsvV264 = 962,
    /// cublasCtbsv_v2
    FuncCublasctbsvV2 = 963,
    /// cublasCtbsv_v2_64
    FuncCublasctbsvV264 = 964,
    /// cublasZtbsv_v2
    FuncCublasztbsvV2 = 965,
    /// cublasZtbsv_v2_64
    FuncCublasztbsvV264 = 966,
    /// cublasSsymv_v2
    FuncCublasssymvV2 = 967,
    /// cublasSsymv_v2_64
    FuncCublasssymvV264 = 968,
    /// cublasDsymv_v2
    FuncCublasdsymvV2 = 969,
    /// cublasDsymv_v2_64
    FuncCublasdsymvV264 = 970,
    /// cublasCsymv_v2
    FuncCublascsymvV2 = 971,
    /// cublasCsymv_v2_64
    FuncCublascsymvV264 = 972,
    /// cublasZsymv_v2
    FuncCublaszsymvV2 = 973,
    /// cublasZsymv_v2_64
    FuncCublaszsymvV264 = 974,
    /// cublasChemv_v2
    FuncCublaschemvV2 = 975,
    /// cublasChemv_v2_64
    FuncCublaschemvV264 = 976,
    /// cublasZhemv_v2
    FuncCublaszhemvV2 = 977,
    /// cublasZhemv_v2_64
    FuncCublaszhemvV264 = 978,
    /// cublasSsbmv_v2
    FuncCublasssbmvV2 = 979,
    /// cublasSsbmv_v2_64
    FuncCublasssbmvV264 = 980,
    /// cublasDsbmv_v2
    FuncCublasdsbmvV2 = 981,
    /// cublasDsbmv_v2_64
    FuncCublasdsbmvV264 = 982,
    /// cublasChbmv_v2
    FuncCublaschbmvV2 = 983,
    /// cublasChbmv_v2_64
    FuncCublaschbmvV264 = 984,
    /// cublasZhbmv_v2
    FuncCublaszhbmvV2 = 985,
    /// cublasZhbmv_v2_64
    FuncCublaszhbmvV264 = 986,
    /// cublasSspmv_v2
    FuncCublassspmvV2 = 987,
    /// cublasSspmv_v2_64
    FuncCublassspmvV264 = 988,
    /// cublasDspmv_v2
    FuncCublasdspmvV2 = 989,
    /// cublasDspmv_v2_64
    FuncCublasdspmvV264 = 990,
    /// cublasChpmv_v2
    FuncCublaschpmvV2 = 991,
    /// cublasChpmv_v2_64
    FuncCublaschpmvV264 = 992,
    /// cublasZhpmv_v2
    FuncCublaszhpmvV2 = 993,
    /// cublasZhpmv_v2_64
    FuncCublaszhpmvV264 = 994,
    /// cublasSger_v2
    FuncCublassgerV2 = 995,
    /// cublasSger_v2_64
    FuncCublassgerV264 = 996,
    /// cublasDger_v2
    FuncCublasdgerV2 = 997,
    /// cublasDger_v2_64
    FuncCublasdgerV264 = 998,
    /// cublasCgeru_v2
    FuncCublascgeruV2 = 999,
    /// cublasCgeru_v2_64
    FuncCublascgeruV264 = 1000,
    /// cublasCgerc_v2
    FuncCublascgercV2 = 1001,
    /// cublasCgerc_v2_64
    FuncCublascgercV264 = 1002,
    /// cublasZgeru_v2
    FuncCublaszgeruV2 = 1003,
    /// cublasZgeru_v2_64
    FuncCublaszgeruV264 = 1004,
    /// cublasZgerc_v2
    FuncCublaszgercV2 = 1005,
    /// cublasZgerc_v2_64
    FuncCublaszgercV264 = 1006,
    /// cublasSsyr_v2
    FuncCublasssyrV2 = 1007,
    /// cublasSsyr_v2_64
    FuncCublasssyrV264 = 1008,
    /// cublasDsyr_v2
    FuncCublasdsyrV2 = 1009,
    /// cublasDsyr_v2_64
    FuncCublasdsyrV264 = 1010,
    /// cublasCsyr_v2
    FuncCublascsyrV2 = 1011,
    /// cublasCsyr_v2_64
    FuncCublascsyrV264 = 1012,
    /// cublasZsyr_v2
    FuncCublaszsyrV2 = 1013,
    /// cublasZsyr_v2_64
    FuncCublaszsyrV264 = 1014,
    /// cublasCher_v2
    FuncCublascherV2 = 1015,
    /// cublasCher_v2_64
    FuncCublascherV264 = 1016,
    /// cublasZher_v2
    FuncCublaszherV2 = 1017,
    /// cublasZher_v2_64
    FuncCublaszherV264 = 1018,
    /// cublasSspr_v2
    FuncCublasssprV2 = 1019,
    /// cublasSspr_v2_64
    FuncCublasssprV264 = 1020,
    /// cublasDspr_v2
    FuncCublasdsprV2 = 1021,
    /// cublasDspr_v2_64
    FuncCublasdsprV264 = 1022,
    /// cublasChpr_v2
    FuncCublaschprV2 = 1023,
    /// cublasChpr_v2_64
    FuncCublaschprV264 = 1024,
    /// cublasZhpr_v2
    FuncCublaszhprV2 = 1025,
    /// cublasZhpr_v2_64
    FuncCublaszhprV264 = 1026,
    /// cublasSsyr2_v2
    FuncCublasssyr2V2 = 1027,
    /// cublasSsyr2_v2_64
    FuncCublasssyr2V264 = 1028,
    /// cublasDsyr2_v2
    FuncCublasdsyr2V2 = 1029,
    /// cublasDsyr2_v2_64
    FuncCublasdsyr2V264 = 1030,
    /// cublasCsyr2_v2
    FuncCublascsyr2V2 = 1031,
    /// cublasCsyr2_v2_64
    FuncCublascsyr2V264 = 1032,
    /// cublasZsyr2_v2
    FuncCublaszsyr2V2 = 1033,
    /// cublasZsyr2_v2_64
    FuncCublaszsyr2V264 = 1034,
    /// cublasCher2_v2
    FuncCublascher2V2 = 1035,
    /// cublasCher2_v2_64
    FuncCublascher2V264 = 1036,
    /// cublasZher2_v2
    FuncCublaszher2V2 = 1037,
    /// cublasZher2_v2_64
    FuncCublaszher2V264 = 1038,
    /// cublasSspr2_v2
    FuncCublassspr2V2 = 1039,
    /// cublasSspr2_v2_64
    FuncCublassspr2V264 = 1040,
    /// cublasDspr2_v2
    FuncCublasdspr2V2 = 1041,
    /// cublasDspr2_v2_64
    FuncCublasdspr2V264 = 1042,
    /// cublasChpr2_v2
    FuncCublaschpr2V2 = 1043,
    /// cublasChpr2_v2_64
    FuncCublaschpr2V264 = 1044,
    /// cublasZhpr2_v2
    FuncCublaszhpr2V2 = 1045,
    /// cublasZhpr2_v2_64
    FuncCublaszhpr2V264 = 1046,
    /// cublasSgemvBatched
    FuncCublassgemvbatched = 1047,
    /// cublasSgemvBatched_64
    FuncCublassgemvbatched64 = 1048,
    /// cublasDgemvBatched
    FuncCublasdgemvbatched = 1049,
    /// cublasDgemvBatched_64
    FuncCublasdgemvbatched64 = 1050,
    /// cublasCgemvBatched
    FuncCublascgemvbatched = 1051,
    /// cublasCgemvBatched_64
    FuncCublascgemvbatched64 = 1052,
    /// cublasZgemvBatched
    FuncCublaszgemvbatched = 1053,
    /// cublasZgemvBatched_64
    FuncCublaszgemvbatched64 = 1054,
    /// cublasSgemvStridedBatched
    FuncCublassgemvstridedbatched = 1055,
    /// cublasSgemvStridedBatched_64
    FuncCublassgemvstridedbatched64 = 1056,
    /// cublasDgemvStridedBatched
    FuncCublasdgemvstridedbatched = 1057,
    /// cublasDgemvStridedBatched_64
    FuncCublasdgemvstridedbatched64 = 1058,
    /// cublasCgemvStridedBatched
    FuncCublascgemvstridedbatched = 1059,
    /// cublasCgemvStridedBatched_64
    FuncCublascgemvstridedbatched64 = 1060,
    /// cublasZgemvStridedBatched
    FuncCublaszgemvstridedbatched = 1061,
    /// cublasZgemvStridedBatched_64
    FuncCublaszgemvstridedbatched64 = 1062,
    /// cublasSgemm_v2
    FuncCublassgemmV2 = 1063,
    /// cublasSgemm_v2_64
    FuncCublassgemmV264 = 1064,
    /// cublasDgemm_v2
    FuncCublasdgemmV2 = 1065,
    /// cublasDgemm_v2_64
    FuncCublasdgemmV264 = 1066,
    /// cublasCgemm_v2
    FuncCublascgemmV2 = 1067,
    /// cublasCgemm_v2_64
    FuncCublascgemmV264 = 1068,
    /// cublasCgemm3m
    FuncCublascgemm3m = 1069,
    /// cublasCgemm3m_64
    FuncCublascgemm3m64 = 1070,
    /// cublasCgemm3mEx
    FuncCublascgemm3mex = 1071,
    /// cublasCgemm3mEx_64
    FuncCublascgemm3mex64 = 1072,
    /// cublasZgemm_v2
    FuncCublaszgemmV2 = 1073,
    /// cublasZgemm_v2_64
    FuncCublaszgemmV264 = 1074,
    /// cublasZgemm3m
    FuncCublaszgemm3m = 1075,
    /// cublasZgemm3m_64
    FuncCublaszgemm3m64 = 1076,
    /// cublasSgemmEx
    FuncCublassgemmex = 1077,
    /// cublasSgemmEx_64
    FuncCublassgemmex64 = 1078,
    /// cublasGemmEx
    FuncCublasgemmex = 1079,
    /// cublasGemmEx_64
    FuncCublasgemmex64 = 1080,
    /// cublasCgemmEx
    FuncCublascgemmex = 1081,
    /// cublasCgemmEx_64
    FuncCublascgemmex64 = 1082,
    /// cublasSsyrk_v2
    FuncCublasssyrkV2 = 1083,
    /// cublasSsyrk_v2_64
    FuncCublasssyrkV264 = 1084,
    /// cublasDsyrk_v2
    FuncCublasdsyrkV2 = 1085,
    /// cublasDsyrk_v2_64
    FuncCublasdsyrkV264 = 1086,
    /// cublasCsyrk_v2
    FuncCublascsyrkV2 = 1087,
    /// cublasCsyrk_v2_64
    FuncCublascsyrkV264 = 1088,
    /// cublasZsyrk_v2
    FuncCublaszsyrkV2 = 1089,
    /// cublasZsyrk_v2_64
    FuncCublaszsyrkV264 = 1090,
    /// cublasCsyrkEx
    FuncCublascsyrkex = 1091,
    /// cublasCsyrkEx_64
    FuncCublascsyrkex64 = 1092,
    /// cublasCsyrk3mEx
    FuncCublascsyrk3mex = 1093,
    /// cublasCsyrk3mEx_64
    FuncCublascsyrk3mex64 = 1094,
    /// cublasCherk_v2
    FuncCublascherkV2 = 1095,
    /// cublasCherk_v2_64
    FuncCublascherkV264 = 1096,
    /// cublasZherk_v2
    FuncCublaszherkV2 = 1097,
    /// cublasZherk_v2_64
    FuncCublaszherkV264 = 1098,
    /// cublasCherkEx
    FuncCublascherkex = 1099,
    /// cublasCherkEx_64
    FuncCublascherkex64 = 1100,
    /// cublasCherk3mEx
    FuncCublascherk3mex = 1101,
    /// cublasCherk3mEx_64
    FuncCublascherk3mex64 = 1102,
    /// cublasSsyr2k_v2
    FuncCublasssyr2kV2 = 1103,
    /// cublasSsyr2k_v2_64
    FuncCublasssyr2kV264 = 1104,
    /// cublasDsyr2k_v2
    FuncCublasdsyr2kV2 = 1105,
    /// cublasDsyr2k_v2_64
    FuncCublasdsyr2kV264 = 1106,
    /// cublasCsyr2k_v2
    FuncCublascsyr2kV2 = 1107,
    /// cublasCsyr2k_v2_64
    FuncCublascsyr2kV264 = 1108,
    /// cublasZsyr2k_v2
    FuncCublaszsyr2kV2 = 1109,
    /// cublasZsyr2k_v2_64
    FuncCublaszsyr2kV264 = 1110,
    /// cublasCher2k_v2
    FuncCublascher2kV2 = 1111,
    /// cublasCher2k_v2_64
    FuncCublascher2kV264 = 1112,
    /// cublasZher2k_v2
    FuncCublaszher2kV2 = 1113,
    /// cublasZher2k_v2_64
    FuncCublaszher2kV264 = 1114,
    /// cublasSsyrkx
    FuncCublasssyrkx = 1115,
    /// cublasSsyrkx_64
    FuncCublasssyrkx64 = 1116,
    /// cublasDsyrkx
    FuncCublasdsyrkx = 1117,
    /// cublasDsyrkx_64
    FuncCublasdsyrkx64 = 1118,
    /// cublasCsyrkx
    FuncCublascsyrkx = 1119,
    /// cublasCsyrkx_64
    FuncCublascsyrkx64 = 1120,
    /// cublasZsyrkx
    FuncCublaszsyrkx = 1121,
    /// cublasZsyrkx_64
    FuncCublaszsyrkx64 = 1122,
    /// cublasCherkx
    FuncCublascherkx = 1123,
    /// cublasCherkx_64
    FuncCublascherkx64 = 1124,
    /// cublasZherkx
    FuncCublaszherkx = 1125,
    /// cublasZherkx_64
    FuncCublaszherkx64 = 1126,
    /// cublasSsymm_v2
    FuncCublasssymmV2 = 1127,
    /// cublasSsymm_v2_64
    FuncCublasssymmV264 = 1128,
    /// cublasDsymm_v2
    FuncCublasdsymmV2 = 1129,
    /// cublasDsymm_v2_64
    FuncCublasdsymmV264 = 1130,
    /// cublasCsymm_v2
    FuncCublascsymmV2 = 1131,
    /// cublasCsymm_v2_64
    FuncCublascsymmV264 = 1132,
    /// cublasZsymm_v2
    FuncCublaszsymmV2 = 1133,
    /// cublasZsymm_v2_64
    FuncCublaszsymmV264 = 1134,
    /// cublasChemm_v2
    FuncCublaschemmV2 = 1135,
    /// cublasChemm_v2_64
    FuncCublaschemmV264 = 1136,
    /// cublasZhemm_v2
    FuncCublaszhemmV2 = 1137,
    /// cublasZhemm_v2_64
    FuncCublaszhemmV264 = 1138,
    /// cublasStrsm_v2
    FuncCublasstrsmV2 = 1139,
    /// cublasStrsm_v2_64
    FuncCublasstrsmV264 = 1140,
    /// cublasDtrsm_v2
    FuncCublasdtrsmV2 = 1141,
    /// cublasDtrsm_v2_64
    FuncCublasdtrsmV264 = 1142,
    /// cublasCtrsm_v2
    FuncCublasctrsmV2 = 1143,
    /// cublasCtrsm_v2_64
    FuncCublasctrsmV264 = 1144,
    /// cublasZtrsm_v2
    FuncCublasztrsmV2 = 1145,
    /// cublasZtrsm_v2_64
    FuncCublasztrsmV264 = 1146,
    /// cublasStrmm_v2
    FuncCublasstrmmV2 = 1147,
    /// cublasStrmm_v2_64
    FuncCublasstrmmV264 = 1148,
    /// cublasDtrmm_v2
    FuncCublasdtrmmV2 = 1149,
    /// cublasDtrmm_v2_64
    FuncCublasdtrmmV264 = 1150,
    /// cublasCtrmm_v2
    FuncCublasctrmmV2 = 1151,
    /// cublasCtrmm_v2_64
    FuncCublasctrmmV264 = 1152,
    /// cublasZtrmm_v2
    FuncCublasztrmmV2 = 1153,
    /// cublasZtrmm_v2_64
    FuncCublasztrmmV264 = 1154,
    /// cublasSgemmBatched
    FuncCublassgemmbatched = 1155,
    /// cublasSgemmBatched_64
    FuncCublassgemmbatched64 = 1156,
    /// cublasDgemmBatched
    FuncCublasdgemmbatched = 1157,
    /// cublasDgemmBatched_64
    FuncCublasdgemmbatched64 = 1158,
    /// cublasCgemmBatched
    FuncCublascgemmbatched = 1159,
    /// cublasCgemmBatched_64
    FuncCublascgemmbatched64 = 1160,
    /// cublasCgemm3mBatched
    FuncCublascgemm3mbatched = 1161,
    /// cublasCgemm3mBatched_64
    FuncCublascgemm3mbatched64 = 1162,
    /// cublasZgemmBatched
    FuncCublaszgemmbatched = 1163,
    /// cublasZgemmBatched_64
    FuncCublaszgemmbatched64 = 1164,
    /// cublasSgemmStridedBatched
    FuncCublassgemmstridedbatched = 1165,
    /// cublasSgemmStridedBatched_64
    FuncCublassgemmstridedbatched64 = 1166,
    /// cublasDgemmStridedBatched
    FuncCublasdgemmstridedbatched = 1167,
    /// cublasDgemmStridedBatched_64
    FuncCublasdgemmstridedbatched64 = 1168,
    /// cublasCgemmStridedBatched
    FuncCublascgemmstridedbatched = 1169,
    /// cublasCgemmStridedBatched_64
    FuncCublascgemmstridedbatched64 = 1170,
    /// cublasCgemm3mStridedBatched
    FuncCublascgemm3mstridedbatched = 1171,
    /// cublasCgemm3mStridedBatched_64
    FuncCublascgemm3mstridedbatched64 = 1172,
    /// cublasZgemmStridedBatched
    FuncCublaszgemmstridedbatched = 1173,
    /// cublasZgemmStridedBatched_64
    FuncCublaszgemmstridedbatched64 = 1174,
    /// cublasGemmBatchedEx
    FuncCublasgemmbatchedex = 1175,
    /// cublasGemmBatchedEx_64
    FuncCublasgemmbatchedex64 = 1176,
    /// cublasGemmStridedBatchedEx
    FuncCublasgemmstridedbatchedex = 1177,
    /// cublasGemmStridedBatchedEx_64
    FuncCublasgemmstridedbatchedex64 = 1178,
    /// cublasSgemmGroupedBatched
    FuncCublassgemmgroupedbatched = 1179,
    /// cublasSgemmGroupedBatched_64
    FuncCublassgemmgroupedbatched64 = 1180,
    /// cublasDgemmGroupedBatched
    FuncCublasdgemmgroupedbatched = 1181,
    /// cublasDgemmGroupedBatched_64
    FuncCublasdgemmgroupedbatched64 = 1182,
    /// cublasGemmGroupedBatchedEx
    FuncCublasgemmgroupedbatchedex = 1183,
    /// cublasGemmGroupedBatchedEx_64
    FuncCublasgemmgroupedbatchedex64 = 1184,
    /// cublasSgeam
    FuncCublassgeam = 1185,
    /// cublasSgeam_64
    FuncCublassgeam64 = 1186,
    /// cublasDgeam
    FuncCublasdgeam = 1187,
    /// cublasDgeam_64
    FuncCublasdgeam64 = 1188,
    /// cublasCgeam
    FuncCublascgeam = 1189,
    /// cublasCgeam_64
    FuncCublascgeam64 = 1190,
    /// cublasZgeam
    FuncCublaszgeam = 1191,
    /// cublasZgeam_64
    FuncCublaszgeam64 = 1192,
    /// cublasStrsmBatched
    FuncCublasstrsmbatched = 1193,
    /// cublasStrsmBatched_64
    FuncCublasstrsmbatched64 = 1194,
    /// cublasDtrsmBatched
    FuncCublasdtrsmbatched = 1195,
    /// cublasDtrsmBatched_64
    FuncCublasdtrsmbatched64 = 1196,
    /// cublasCtrsmBatched
    FuncCublasctrsmbatched = 1197,
    /// cublasCtrsmBatched_64
    FuncCublasctrsmbatched64 = 1198,
    /// cublasZtrsmBatched
    FuncCublasztrsmbatched = 1199,
    /// cublasZtrsmBatched_64
    FuncCublasztrsmbatched64 = 1200,
    /// cublasSdgmm
    FuncCublassdgmm = 1201,
    /// cublasSdgmm_64
    FuncCublassdgmm64 = 1202,
    /// cublasDdgmm
    FuncCublasddgmm = 1203,
    /// cublasDdgmm_64
    FuncCublasddgmm64 = 1204,
    /// cublasCdgmm
    FuncCublascdgmm = 1205,
    /// cublasCdgmm_64
    FuncCublascdgmm64 = 1206,
    /// cublasZdgmm
    FuncCublaszdgmm = 1207,
    /// cublasZdgmm_64
    FuncCublaszdgmm64 = 1208,
    /// cublasSmatinvBatched
    FuncCublassmatinvbatched = 1209,
    /// cublasDmatinvBatched
    FuncCublasdmatinvbatched = 1210,
    /// cublasCmatinvBatched
    FuncCublascmatinvbatched = 1211,
    /// cublasZmatinvBatched
    FuncCublaszmatinvbatched = 1212,
    /// cublasSgeqrfBatched
    FuncCublassgeqrfbatched = 1213,
    /// cublasDgeqrfBatched
    FuncCublasdgeqrfbatched = 1214,
    /// cublasCgeqrfBatched
    FuncCublascgeqrfbatched = 1215,
    /// cublasZgeqrfBatched
    FuncCublaszgeqrfbatched = 1216,
    /// cublasSgelsBatched
    FuncCublassgelsbatched = 1217,
    /// cublasDgelsBatched
    FuncCublasdgelsbatched = 1218,
    /// cublasCgelsBatched
    FuncCublascgelsbatched = 1219,
    /// cublasZgelsBatched
    FuncCublaszgelsbatched = 1220,
    /// cublasStpttr
    FuncCublasstpttr = 1221,
    /// cublasDtpttr
    FuncCublasdtpttr = 1222,
    /// cublasCtpttr
    FuncCublasctpttr = 1223,
    /// cublasZtpttr
    FuncCublasztpttr = 1224,
    /// cublasStrttp
    FuncCublasstrttp = 1225,
    /// cublasDtrttp
    FuncCublasdtrttp = 1226,
    /// cublasCtrttp
    FuncCublasctrttp = 1227,
    /// cublasZtrttp
    FuncCublasztrttp = 1228,
    /// cublasSgetrfBatched
    FuncCublassgetrfbatched = 1229,
    /// cublasDgetrfBatched
    FuncCublasdgetrfbatched = 1230,
    /// cublasCgetrfBatched
    FuncCublascgetrfbatched = 1231,
    /// cublasZgetrfBatched
    FuncCublaszgetrfbatched = 1232,
    /// cublasSgetriBatched
    FuncCublassgetribatched = 1233,
    /// cublasDgetriBatched
    FuncCublasdgetribatched = 1234,
    /// cublasCgetriBatched
    FuncCublascgetribatched = 1235,
    /// cublasZgetriBatched
    FuncCublaszgetribatched = 1236,
    /// cublasSgetrsBatched
    FuncCublassgetrsbatched = 1237,
    /// cublasDgetrsBatched
    FuncCublasdgetrsbatched = 1238,
    /// cublasCgetrsBatched
    FuncCublascgetrsbatched = 1239,
    /// cublasZgetrsBatched
    FuncCublaszgetrsbatched = 1240,
    /// cublasUint8gemmBias
    FuncCublasuint8gemmbias = 1241,
    /// cublasLtCreate
    FuncCublasltcreate = 1242,
    /// cublasLtDestroy
    FuncCublasltdestroy = 1243,
    /// cublasLtGetStatusName
    FuncCublasltgetstatusname = 1244,
    /// cublasLtGetStatusString
    FuncCublasltgetstatusstring = 1245,
    /// cublasLtGetVersion
    FuncCublasltgetversion = 1246,
    /// cublasLtGetCudartVersion
    FuncCublasltgetcudartversion = 1247,
    /// cublasLtGetProperty
    FuncCublasltgetproperty = 1248,
    /// cublasLtHeuristicsCacheGetCapacity
    FuncCublasltheuristicscachegetcapacity = 1249,
    /// cublasLtHeuristicsCacheSetCapacity
    FuncCublasltheuristicscachesetcapacity = 1250,
    /// cublasLtDisableCpuInstructionsSetMask
    FuncCublasltdisablecpuinstructionssetmask = 1251,
    /// cublasLtMatmul
    FuncCublasltmatmul = 1252,
    /// cublasLtMatrixTransform
    FuncCublasltmatrixtransform = 1253,
    /// cublasLtMatrixLayoutInit_internal
    FuncCublasltmatrixlayoutinitInternal = 1254,
    /// cublasLtMatrixLayoutCreate
    FuncCublasltmatrixlayoutcreate = 1255,
    /// cublasLtMatrixLayoutDestroy
    FuncCublasltmatrixlayoutdestroy = 1256,
    /// cublasLtMatrixLayoutSetAttribute
    FuncCublasltmatrixlayoutsetattribute = 1257,
    /// cublasLtMatrixLayoutGetAttribute
    FuncCublasltmatrixlayoutgetattribute = 1258,
    /// cublasLtMatmulDescInit_internal
    FuncCublasltmatmuldescinitInternal = 1259,
    /// cublasLtMatmulDescCreate
    FuncCublasltmatmuldesccreate = 1260,
    /// cublasLtMatmulDescDestroy
    FuncCublasltmatmuldescdestroy = 1261,
    /// cublasLtMatmulDescSetAttribute
    FuncCublasltmatmuldescsetattribute = 1262,
    /// cublasLtMatmulDescGetAttribute
    FuncCublasltmatmuldescgetattribute = 1263,
    /// cublasLtMatrixTransformDescInit_internal
    FuncCublasltmatrixtransformdescinitInternal = 1264,
    /// cublasLtMatrixTransformDescCreate
    FuncCublasltmatrixtransformdesccreate = 1265,
    /// cublasLtMatrixTransformDescDestroy
    FuncCublasltmatrixtransformdescdestroy = 1266,
    /// cublasLtMatrixTransformDescSetAttribute
    FuncCublasltmatrixtransformdescsetattribute = 1267,
    /// cublasLtMatrixTransformDescGetAttribute
    FuncCublasltmatrixtransformdescgetattribute = 1268,
    /// cublasLtMatmulPreferenceInit_internal
    FuncCublasltmatmulpreferenceinitInternal = 1269,
    /// cublasLtMatmulPreferenceCreate
    FuncCublasltmatmulpreferencecreate = 1270,
    /// cublasLtMatmulPreferenceDestroy
    FuncCublasltmatmulpreferencedestroy = 1271,
    /// cublasLtMatmulPreferenceSetAttribute
    FuncCublasltmatmulpreferencesetattribute = 1272,
    /// cublasLtMatmulPreferenceGetAttribute
    FuncCublasltmatmulpreferencegetattribute = 1273,
    /// cublasLtMatmulAlgoGetHeuristic
    FuncCublasltmatmulalgogetheuristic = 1274,
    /// cublasLtMatmulAlgoGetIds
    FuncCublasltmatmulalgogetids = 1275,
    /// cublasLtMatmulAlgoInit
    FuncCublasltmatmulalgoinit = 1276,
    /// cublasLtMatmulAlgoCheck
    FuncCublasltmatmulalgocheck = 1277,
    /// cublasLtMatmulAlgoCapGetAttribute
    FuncCublasltmatmulalgocapgetattribute = 1278,
    /// cublasLtMatmulAlgoConfigSetAttribute
    FuncCublasltmatmulalgoconfigsetattribute = 1279,
    /// cublasLtMatmulAlgoConfigGetAttribute
    FuncCublasltmatmulalgoconfiggetattribute = 1280,
    /// cublasLtLoggerSetCallback
    FuncCublasltloggersetcallback = 1281,
    /// cublasLtLoggerSetFile
    FuncCublasltloggersetfile = 1282,
    /// cublasLtLoggerOpenFile
    FuncCublasltloggeropenfile = 1283,
    /// cublasLtLoggerSetLevel
    FuncCublasltloggersetlevel = 1284,
    /// cublasLtLoggerSetMask
    FuncCublasltloggersetmask = 1285,
    /// cublasLtLoggerForceDisable
    FuncCublasltloggerforcedisable = 1286,
    /// nvmlInit_v2
    FuncNvmlinitV2 = 1287,
    /// nvmlInitWithFlags
    FuncNvmlinitwithflags = 1288,
    /// nvmlShutdown
    FuncNvmlshutdown = 1289,
    /// nvmlErrorString
    FuncNvmlerrorstring = 1290,
    /// nvmlSystemGetDriverVersion
    FuncNvmlsystemgetdriverversion = 1291,
    /// nvmlSystemGetNVMLVersion
    FuncNvmlsystemgetnvmlversion = 1292,
    /// nvmlSystemGetCudaDriverVersion
    FuncNvmlsystemgetcudadriverversion = 1293,
    /// nvmlSystemGetCudaDriverVersion_v2
    FuncNvmlsystemgetcudadriverversionV2 = 1294,
    /// nvmlSystemGetProcessName
    FuncNvmlsystemgetprocessname = 1295,
    /// nvmlSystemGetHicVersion
    FuncNvmlsystemgethicversion = 1296,
    /// nvmlSystemGetTopologyGpuSet
    FuncNvmlsystemgettopologygpuset = 1297,
    /// nvmlUnitGetCount
    FuncNvmlunitgetcount = 1298,
    /// nvmlUnitGetHandleByIndex
    FuncNvmlunitgethandlebyindex = 1299,
    /// nvmlUnitGetUnitInfo
    FuncNvmlunitgetunitinfo = 1300,
    /// nvmlUnitGetLedState
    FuncNvmlunitgetledstate = 1301,
    /// nvmlUnitGetPsuInfo
    FuncNvmlunitgetpsuinfo = 1302,
    /// nvmlUnitGetTemperature
    FuncNvmlunitgettemperature = 1303,
    /// nvmlUnitGetFanSpeedInfo
    FuncNvmlunitgetfanspeedinfo = 1304,
    /// nvmlUnitGetDevices
    FuncNvmlunitgetdevices = 1305,
    /// nvmlDeviceGetCount_v2
    FuncNvmldevicegetcountV2 = 1306,
    /// nvmlDeviceGetAttributes_v2
    FuncNvmldevicegetattributesV2 = 1307,
    /// nvmlDeviceGetHandleByIndex_v2
    FuncNvmldevicegethandlebyindexV2 = 1308,
    /// nvmlDeviceGetHandleBySerial
    FuncNvmldevicegethandlebyserial = 1309,
    /// nvmlDeviceGetHandleByUUID
    FuncNvmldevicegethandlebyuuid = 1310,
    /// nvmlDeviceGetHandleByPciBusId_v2
    FuncNvmldevicegethandlebypcibusidV2 = 1311,
    /// nvmlDeviceGetName
    FuncNvmldevicegetname = 1312,
    /// nvmlDeviceGetBrand
    FuncNvmldevicegetbrand = 1313,
    /// nvmlDeviceGetIndex
    FuncNvmldevicegetindex = 1314,
    /// nvmlDeviceGetSerial
    FuncNvmldevicegetserial = 1315,
    /// nvmlDeviceGetModuleId
    FuncNvmldevicegetmoduleid = 1316,
    /// nvmlDeviceGetC2cModeInfoV
    FuncNvmldevicegetc2cmodeinfov = 1317,
    /// nvmlDeviceGetMemoryAffinity
    FuncNvmldevicegetmemoryaffinity = 1318,
    /// nvmlDeviceGetCpuAffinityWithinScope
    FuncNvmldevicegetcpuaffinitywithinscope = 1319,
    /// nvmlDeviceGetCpuAffinity
    FuncNvmldevicegetcpuaffinity = 1320,
    /// nvmlDeviceSetCpuAffinity
    FuncNvmldevicesetcpuaffinity = 1321,
    /// nvmlDeviceClearCpuAffinity
    FuncNvmldeviceclearcpuaffinity = 1322,
    /// nvmlDeviceGetNumaNodeId
    FuncNvmldevicegetnumanodeid = 1323,
    /// nvmlDeviceGetTopologyCommonAncestor
    FuncNvmldevicegettopologycommonancestor = 1324,
    /// nvmlDeviceGetTopologyNearestGpus
    FuncNvmldevicegettopologynearestgpus = 1325,
    /// nvmlDeviceGetP2PStatus
    FuncNvmldevicegetp2pstatus = 1326,
    /// nvmlDeviceGetUUID
    FuncNvmldevicegetuuid = 1327,
    /// nvmlDeviceGetMinorNumber
    FuncNvmldevicegetminornumber = 1328,
    /// nvmlDeviceGetBoardPartNumber
    FuncNvmldevicegetboardpartnumber = 1329,
    /// nvmlDeviceGetInforomVersion
    FuncNvmldevicegetinforomversion = 1330,
    /// nvmlDeviceGetInforomImageVersion
    FuncNvmldevicegetinforomimageversion = 1331,
    /// nvmlDeviceGetInforomConfigurationChecksum
    FuncNvmldevicegetinforomconfigurationchecksum = 1332,
    /// nvmlDeviceValidateInforom
    FuncNvmldevicevalidateinforom = 1333,
    /// nvmlDeviceGetLastBBXFlushTime
    FuncNvmldevicegetlastbbxflushtime = 1334,
    /// nvmlDeviceGetDisplayMode
    FuncNvmldevicegetdisplaymode = 1335,
    /// nvmlDeviceGetDisplayActive
    FuncNvmldevicegetdisplayactive = 1336,
    /// nvmlDeviceGetPersistenceMode
    FuncNvmldevicegetpersistencemode = 1337,
    /// nvmlDeviceGetPciInfoExt
    FuncNvmldevicegetpciinfoext = 1338,
    /// nvmlDeviceGetPciInfo_v3
    FuncNvmldevicegetpciinfoV3 = 1339,
    /// nvmlDeviceGetMaxPcieLinkGeneration
    FuncNvmldevicegetmaxpcielinkgeneration = 1340,
    /// nvmlDeviceGetGpuMaxPcieLinkGeneration
    FuncNvmldevicegetgpumaxpcielinkgeneration = 1341,
    /// nvmlDeviceGetMaxPcieLinkWidth
    FuncNvmldevicegetmaxpcielinkwidth = 1342,
    /// nvmlDeviceGetCurrPcieLinkGeneration
    FuncNvmldevicegetcurrpcielinkgeneration = 1343,
    /// nvmlDeviceGetCurrPcieLinkWidth
    FuncNvmldevicegetcurrpcielinkwidth = 1344,
    /// nvmlDeviceGetPcieThroughput
    FuncNvmldevicegetpciethroughput = 1345,
    /// nvmlDeviceGetPcieReplayCounter
    FuncNvmldevicegetpciereplaycounter = 1346,
    /// nvmlDeviceGetClockInfo
    FuncNvmldevicegetclockinfo = 1347,
    /// nvmlDeviceGetMaxClockInfo
    FuncNvmldevicegetmaxclockinfo = 1348,
    /// nvmlDeviceGetGpcClkVfOffset
    FuncNvmldevicegetgpcclkvfoffset = 1349,
    /// nvmlDeviceGetApplicationsClock
    FuncNvmldevicegetapplicationsclock = 1350,
    /// nvmlDeviceGetDefaultApplicationsClock
    FuncNvmldevicegetdefaultapplicationsclock = 1351,
    /// nvmlDeviceGetClock
    FuncNvmldevicegetclock = 1352,
    /// nvmlDeviceGetMaxCustomerBoostClock
    FuncNvmldevicegetmaxcustomerboostclock = 1353,
    /// nvmlDeviceGetSupportedMemoryClocks
    FuncNvmldevicegetsupportedmemoryclocks = 1354,
    /// nvmlDeviceGetSupportedGraphicsClocks
    FuncNvmldevicegetsupportedgraphicsclocks = 1355,
    /// nvmlDeviceGetAutoBoostedClocksEnabled
    FuncNvmldevicegetautoboostedclocksenabled = 1356,
    /// nvmlDeviceGetFanSpeed
    FuncNvmldevicegetfanspeed = 1357,
    /// nvmlDeviceGetFanSpeed_v2
    FuncNvmldevicegetfanspeedV2 = 1358,
    /// nvmlDeviceGetTargetFanSpeed
    FuncNvmldevicegettargetfanspeed = 1359,
    /// nvmlDeviceGetMinMaxFanSpeed
    FuncNvmldevicegetminmaxfanspeed = 1360,
    /// nvmlDeviceGetFanControlPolicy_v2
    FuncNvmldevicegetfancontrolpolicyV2 = 1361,
    /// nvmlDeviceGetNumFans
    FuncNvmldevicegetnumfans = 1362,
    /// nvmlDeviceGetTemperature
    FuncNvmldevicegettemperature = 1363,
    /// nvmlDeviceGetTemperatureThreshold
    FuncNvmldevicegettemperaturethreshold = 1364,
    /// nvmlDeviceGetThermalSettings
    FuncNvmldevicegetthermalsettings = 1365,
    /// nvmlDeviceGetPerformanceState
    FuncNvmldevicegetperformancestate = 1366,
    /// nvmlDeviceGetCurrentClocksEventReasons
    FuncNvmldevicegetcurrentclockseventreasons = 1367,
    /// nvmlDeviceGetCurrentClocksThrottleReasons
    FuncNvmldevicegetcurrentclocksthrottlereasons = 1368,
    /// nvmlDeviceGetSupportedClocksEventReasons
    FuncNvmldevicegetsupportedclockseventreasons = 1369,
    /// nvmlDeviceGetSupportedClocksThrottleReasons
    FuncNvmldevicegetsupportedclocksthrottlereasons = 1370,
    /// nvmlDeviceGetPowerState
    FuncNvmldevicegetpowerstate = 1371,
    /// nvmlDeviceGetDynamicPstatesInfo
    FuncNvmldevicegetdynamicpstatesinfo = 1372,
    /// nvmlDeviceGetMemClkVfOffset
    FuncNvmldevicegetmemclkvfoffset = 1373,
    /// nvmlDeviceGetMinMaxClockOfPState
    FuncNvmldevicegetminmaxclockofpstate = 1374,
    /// nvmlDeviceGetSupportedPerformanceStates
    FuncNvmldevicegetsupportedperformancestates = 1375,
    /// nvmlDeviceGetGpcClkMinMaxVfOffset
    FuncNvmldevicegetgpcclkminmaxvfoffset = 1376,
    /// nvmlDeviceGetMemClkMinMaxVfOffset
    FuncNvmldevicegetmemclkminmaxvfoffset = 1377,
    /// nvmlDeviceGetClockOffsets
    FuncNvmldevicegetclockoffsets = 1378,
    /// nvmlDeviceSetClockOffsets
    FuncNvmldevicesetclockoffsets = 1379,
    /// nvmlDeviceGetPowerManagementMode
    FuncNvmldevicegetpowermanagementmode = 1380,
    /// nvmlDeviceGetPowerManagementLimit
    FuncNvmldevicegetpowermanagementlimit = 1381,
    /// nvmlDeviceGetPowerManagementLimitConstraints
    FuncNvmldevicegetpowermanagementlimitconstraints = 1382,
    /// nvmlDeviceGetPowerManagementDefaultLimit
    FuncNvmldevicegetpowermanagementdefaultlimit = 1383,
    /// nvmlDeviceGetPowerUsage
    FuncNvmldevicegetpowerusage = 1384,
    /// nvmlDeviceGetTotalEnergyConsumption
    FuncNvmldevicegettotalenergyconsumption = 1385,
    /// nvmlDeviceGetEnforcedPowerLimit
    FuncNvmldevicegetenforcedpowerlimit = 1386,
    /// nvmlDeviceGetGpuOperationMode
    FuncNvmldevicegetgpuoperationmode = 1387,
    /// nvmlDeviceGetMemoryInfo
    FuncNvmldevicegetmemoryinfo = 1388,
    /// nvmlDeviceGetMemoryInfo_v2
    FuncNvmldevicegetmemoryinfoV2 = 1389,
    /// nvmlDeviceGetComputeMode
    FuncNvmldevicegetcomputemode = 1390,
    /// nvmlDeviceGetCudaComputeCapability
    FuncNvmldevicegetcudacomputecapability = 1391,
    /// nvmlDeviceGetEccMode
    FuncNvmldevicegeteccmode = 1392,
    /// nvmlDeviceGetDefaultEccMode
    FuncNvmldevicegetdefaulteccmode = 1393,
    /// nvmlDeviceGetBoardId
    FuncNvmldevicegetboardid = 1394,
    /// nvmlDeviceGetMultiGpuBoard
    FuncNvmldevicegetmultigpuboard = 1395,
    /// nvmlDeviceGetTotalEccErrors
    FuncNvmldevicegettotaleccerrors = 1396,
    /// nvmlDeviceGetDetailedEccErrors
    FuncNvmldevicegetdetailedeccerrors = 1397,
    /// nvmlDeviceGetMemoryErrorCounter
    FuncNvmldevicegetmemoryerrorcounter = 1398,
    /// nvmlDeviceGetUtilizationRates
    FuncNvmldevicegetutilizationrates = 1399,
    /// nvmlDeviceGetEncoderUtilization
    FuncNvmldevicegetencoderutilization = 1400,
    /// nvmlDeviceGetEncoderCapacity
    FuncNvmldevicegetencodercapacity = 1401,
    /// nvmlDeviceGetEncoderStats
    FuncNvmldevicegetencoderstats = 1402,
    /// nvmlDeviceGetEncoderSessions
    FuncNvmldevicegetencodersessions = 1403,
    /// nvmlDeviceGetDecoderUtilization
    FuncNvmldevicegetdecoderutilization = 1404,
    /// nvmlDeviceGetJpgUtilization
    FuncNvmldevicegetjpgutilization = 1405,
    /// nvmlDeviceGetOfaUtilization
    FuncNvmldevicegetofautilization = 1406,
    /// nvmlDeviceGetFBCStats
    FuncNvmldevicegetfbcstats = 1407,
    /// nvmlDeviceGetFBCSessions
    FuncNvmldevicegetfbcsessions = 1408,
    /// nvmlDeviceGetDriverModel_v2
    FuncNvmldevicegetdrivermodelV2 = 1409,
    /// nvmlDeviceGetVbiosVersion
    FuncNvmldevicegetvbiosversion = 1410,
    /// nvmlDeviceGetBridgeChipInfo
    FuncNvmldevicegetbridgechipinfo = 1411,
    /// nvmlDeviceGetComputeRunningProcesses_v3
    FuncNvmldevicegetcomputerunningprocessesV3 = 1412,
    /// nvmlDeviceGetGraphicsRunningProcesses_v3
    FuncNvmldevicegetgraphicsrunningprocessesV3 = 1413,
    /// nvmlDeviceGetMPSComputeRunningProcesses_v3
    FuncNvmldevicegetmpscomputerunningprocessesV3 = 1414,
    /// nvmlDeviceGetRunningProcessDetailList
    FuncNvmldevicegetrunningprocessdetaillist = 1415,
    /// nvmlDeviceOnSameBoard
    FuncNvmldeviceonsameboard = 1416,
    /// nvmlDeviceGetAPIRestriction
    FuncNvmldevicegetapirestriction = 1417,
    /// nvmlDeviceGetSamples
    FuncNvmldevicegetsamples = 1418,
    /// nvmlDeviceGetBAR1MemoryInfo
    FuncNvmldevicegetbar1memoryinfo = 1419,
    /// nvmlDeviceGetViolationStatus
    FuncNvmldevicegetviolationstatus = 1420,
    /// nvmlDeviceGetIrqNum
    FuncNvmldevicegetirqnum = 1421,
    /// nvmlDeviceGetNumGpuCores
    FuncNvmldevicegetnumgpucores = 1422,
    /// nvmlDeviceGetPowerSource
    FuncNvmldevicegetpowersource = 1423,
    /// nvmlDeviceGetMemoryBusWidth
    FuncNvmldevicegetmemorybuswidth = 1424,
    /// nvmlDeviceGetPcieLinkMaxSpeed
    FuncNvmldevicegetpcielinkmaxspeed = 1425,
    /// nvmlDeviceGetPcieSpeed
    FuncNvmldevicegetpciespeed = 1426,
    /// nvmlDeviceGetAdaptiveClockInfoStatus
    FuncNvmldevicegetadaptiveclockinfostatus = 1427,
    /// nvmlDeviceGetBusType
    FuncNvmldevicegetbustype = 1428,
    /// nvmlDeviceGetGpuFabricInfo
    FuncNvmldevicegetgpufabricinfo = 1429,
    /// nvmlDeviceGetGpuFabricInfoV
    FuncNvmldevicegetgpufabricinfov = 1430,
    /// nvmlSystemGetConfComputeCapabilities
    FuncNvmlsystemgetconfcomputecapabilities = 1431,
    /// nvmlSystemGetConfComputeState
    FuncNvmlsystemgetconfcomputestate = 1432,
    /// nvmlDeviceGetConfComputeMemSizeInfo
    FuncNvmldevicegetconfcomputememsizeinfo = 1433,
    /// nvmlSystemGetConfComputeGpusReadyState
    FuncNvmlsystemgetconfcomputegpusreadystate = 1434,
    /// nvmlDeviceGetConfComputeProtectedMemoryUsage
    FuncNvmldevicegetconfcomputeprotectedmemoryusage = 1435,
    /// nvmlDeviceGetConfComputeGpuCertificate
    FuncNvmldevicegetconfcomputegpucertificate = 1436,
    /// nvmlDeviceGetConfComputeGpuAttestationReport
    FuncNvmldevicegetconfcomputegpuattestationreport = 1437,
    /// nvmlSystemGetConfComputeKeyRotationThresholdInfo
    FuncNvmlsystemgetconfcomputekeyrotationthresholdinfo = 1438,
    /// nvmlDeviceSetConfComputeUnprotectedMemSize
    FuncNvmldevicesetconfcomputeunprotectedmemsize = 1439,
    /// nvmlSystemSetConfComputeGpusReadyState
    FuncNvmlsystemsetconfcomputegpusreadystate = 1440,
    /// nvmlSystemSetConfComputeKeyRotationThresholdInfo
    FuncNvmlsystemsetconfcomputekeyrotationthresholdinfo = 1441,
    /// nvmlSystemGetConfComputeSettings
    FuncNvmlsystemgetconfcomputesettings = 1442,
    /// nvmlDeviceGetGspFirmwareVersion
    FuncNvmldevicegetgspfirmwareversion = 1443,
    /// nvmlDeviceGetGspFirmwareMode
    FuncNvmldevicegetgspfirmwaremode = 1444,
    /// nvmlDeviceGetAccountingMode
    FuncNvmldevicegetaccountingmode = 1445,
    /// nvmlDeviceGetAccountingStats
    FuncNvmldevicegetaccountingstats = 1446,
    /// nvmlDeviceGetAccountingPids
    FuncNvmldevicegetaccountingpids = 1447,
    /// nvmlDeviceGetAccountingBufferSize
    FuncNvmldevicegetaccountingbuffersize = 1448,
    /// nvmlDeviceGetRetiredPages
    FuncNvmldevicegetretiredpages = 1449,
    /// nvmlDeviceGetRetiredPages_v2
    FuncNvmldevicegetretiredpagesV2 = 1450,
    /// nvmlDeviceGetRetiredPagesPendingStatus
    FuncNvmldevicegetretiredpagespendingstatus = 1451,
    /// nvmlDeviceGetRemappedRows
    FuncNvmldevicegetremappedrows = 1452,
    /// nvmlDeviceGetRowRemapperHistogram
    FuncNvmldevicegetrowremapperhistogram = 1453,
    /// nvmlDeviceGetArchitecture
    FuncNvmldevicegetarchitecture = 1454,
    /// nvmlDeviceGetClkMonStatus
    FuncNvmldevicegetclkmonstatus = 1455,
    /// nvmlDeviceGetProcessUtilization
    FuncNvmldevicegetprocessutilization = 1456,
    /// nvmlDeviceGetProcessesUtilizationInfo
    FuncNvmldevicegetprocessesutilizationinfo = 1457,
    /// nvmlUnitSetLedState
    FuncNvmlunitsetledstate = 1458,
    /// nvmlDeviceSetPersistenceMode
    FuncNvmldevicesetpersistencemode = 1459,
    /// nvmlDeviceSetComputeMode
    FuncNvmldevicesetcomputemode = 1460,
    /// nvmlDeviceSetEccMode
    FuncNvmldeviceseteccmode = 1461,
    /// nvmlDeviceClearEccErrorCounts
    FuncNvmldevicecleareccerrorcounts = 1462,
    /// nvmlDeviceSetDriverModel
    FuncNvmldevicesetdrivermodel = 1463,
    /// nvmlDeviceSetGpuLockedClocks
    FuncNvmldevicesetgpulockedclocks = 1464,
    /// nvmlDeviceResetGpuLockedClocks
    FuncNvmldeviceresetgpulockedclocks = 1465,
    /// nvmlDeviceSetMemoryLockedClocks
    FuncNvmldevicesetmemorylockedclocks = 1466,
    /// nvmlDeviceResetMemoryLockedClocks
    FuncNvmldeviceresetmemorylockedclocks = 1467,
    /// nvmlDeviceSetApplicationsClocks
    FuncNvmldevicesetapplicationsclocks = 1468,
    /// nvmlDeviceResetApplicationsClocks
    FuncNvmldeviceresetapplicationsclocks = 1469,
    /// nvmlDeviceSetAutoBoostedClocksEnabled
    FuncNvmldevicesetautoboostedclocksenabled = 1470,
    /// nvmlDeviceSetDefaultAutoBoostedClocksEnabled
    FuncNvmldevicesetdefaultautoboostedclocksenabled = 1471,
    /// nvmlDeviceSetDefaultFanSpeed_v2
    FuncNvmldevicesetdefaultfanspeedV2 = 1472,
    /// nvmlDeviceSetFanControlPolicy
    FuncNvmldevicesetfancontrolpolicy = 1473,
    /// nvmlDeviceSetTemperatureThreshold
    FuncNvmldevicesettemperaturethreshold = 1474,
    /// nvmlDeviceSetPowerManagementLimit
    FuncNvmldevicesetpowermanagementlimit = 1475,
    /// nvmlDeviceSetGpuOperationMode
    FuncNvmldevicesetgpuoperationmode = 1476,
    /// nvmlDeviceSetAPIRestriction
    FuncNvmldevicesetapirestriction = 1477,
    /// nvmlDeviceSetFanSpeed_v2
    FuncNvmldevicesetfanspeedV2 = 1478,
    /// nvmlDeviceSetGpcClkVfOffset
    FuncNvmldevicesetgpcclkvfoffset = 1479,
    /// nvmlDeviceSetMemClkVfOffset
    FuncNvmldevicesetmemclkvfoffset = 1480,
    /// nvmlDeviceSetAccountingMode
    FuncNvmldevicesetaccountingmode = 1481,
    /// nvmlDeviceClearAccountingPids
    FuncNvmldeviceclearaccountingpids = 1482,
    /// nvmlDeviceGetNvLinkState
    FuncNvmldevicegetnvlinkstate = 1483,
    /// nvmlDeviceGetNvLinkVersion
    FuncNvmldevicegetnvlinkversion = 1484,
    /// nvmlDeviceGetNvLinkCapability
    FuncNvmldevicegetnvlinkcapability = 1485,
    /// nvmlDeviceGetNvLinkRemotePciInfo_v2
    FuncNvmldevicegetnvlinkremotepciinfoV2 = 1486,
    /// nvmlDeviceGetNvLinkErrorCounter
    FuncNvmldevicegetnvlinkerrorcounter = 1487,
    /// nvmlDeviceResetNvLinkErrorCounters
    FuncNvmldeviceresetnvlinkerrorcounters = 1488,
    /// nvmlDeviceSetNvLinkUtilizationControl
    FuncNvmldevicesetnvlinkutilizationcontrol = 1489,
    /// nvmlDeviceGetNvLinkUtilizationControl
    FuncNvmldevicegetnvlinkutilizationcontrol = 1490,
    /// nvmlDeviceGetNvLinkUtilizationCounter
    FuncNvmldevicegetnvlinkutilizationcounter = 1491,
    /// nvmlDeviceFreezeNvLinkUtilizationCounter
    FuncNvmldevicefreezenvlinkutilizationcounter = 1492,
    /// nvmlDeviceResetNvLinkUtilizationCounter
    FuncNvmldeviceresetnvlinkutilizationcounter = 1493,
    /// nvmlDeviceGetNvLinkRemoteDeviceType
    FuncNvmldevicegetnvlinkremotedevicetype = 1494,
    /// nvmlEventSetCreate
    FuncNvmleventsetcreate = 1495,
    /// nvmlDeviceRegisterEvents
    FuncNvmldeviceregisterevents = 1496,
    /// nvmlDeviceGetSupportedEventTypes
    FuncNvmldevicegetsupportedeventtypes = 1497,
    /// nvmlEventSetWait_v2
    FuncNvmleventsetwaitV2 = 1498,
    /// nvmlEventSetFree
    FuncNvmleventsetfree = 1499,
    /// nvmlDeviceModifyDrainState
    FuncNvmldevicemodifydrainstate = 1500,
    /// nvmlDeviceQueryDrainState
    FuncNvmldevicequerydrainstate = 1501,
    /// nvmlDeviceRemoveGpu_v2
    FuncNvmldeviceremovegpuV2 = 1502,
    /// nvmlDeviceDiscoverGpus
    FuncNvmldevicediscovergpus = 1503,
    /// nvmlDeviceGetFieldValues
    FuncNvmldevicegetfieldvalues = 1504,
    /// nvmlDeviceClearFieldValues
    FuncNvmldeviceclearfieldvalues = 1505,
    /// nvmlDeviceGetVirtualizationMode
    FuncNvmldevicegetvirtualizationmode = 1506,
    /// nvmlDeviceGetHostVgpuMode
    FuncNvmldevicegethostvgpumode = 1507,
    /// nvmlDeviceSetVirtualizationMode
    FuncNvmldevicesetvirtualizationmode = 1508,
    /// nvmlDeviceGetVgpuHeterogeneousMode
    FuncNvmldevicegetvgpuheterogeneousmode = 1509,
    /// nvmlDeviceSetVgpuHeterogeneousMode
    FuncNvmldevicesetvgpuheterogeneousmode = 1510,
    /// nvmlVgpuInstanceGetPlacementId
    FuncNvmlvgpuinstancegetplacementid = 1511,
    /// nvmlDeviceGetVgpuTypeSupportedPlacements
    FuncNvmldevicegetvgputypesupportedplacements = 1512,
    /// nvmlDeviceGetVgpuTypeCreatablePlacements
    FuncNvmldevicegetvgputypecreatableplacements = 1513,
    /// nvmlVgpuTypeGetGspHeapSize
    FuncNvmlvgputypegetgspheapsize = 1514,
    /// nvmlVgpuTypeGetFbReservation
    FuncNvmlvgputypegetfbreservation = 1515,
    /// nvmlDeviceSetVgpuCapabilities
    FuncNvmldevicesetvgpucapabilities = 1516,
    /// nvmlDeviceGetGridLicensableFeatures_v4
    FuncNvmldevicegetgridlicensablefeaturesV4 = 1517,
    /// nvmlGetVgpuDriverCapabilities
    FuncNvmlgetvgpudrivercapabilities = 1518,
    /// nvmlDeviceGetVgpuCapabilities
    FuncNvmldevicegetvgpucapabilities = 1519,
    /// nvmlDeviceGetSupportedVgpus
    FuncNvmldevicegetsupportedvgpus = 1520,
    /// nvmlDeviceGetCreatableVgpus
    FuncNvmldevicegetcreatablevgpus = 1521,
    /// nvmlVgpuTypeGetClass
    FuncNvmlvgputypegetclass = 1522,
    /// nvmlVgpuTypeGetName
    FuncNvmlvgputypegetname = 1523,
    /// nvmlVgpuTypeGetGpuInstanceProfileId
    FuncNvmlvgputypegetgpuinstanceprofileid = 1524,
    /// nvmlVgpuTypeGetDeviceID
    FuncNvmlvgputypegetdeviceid = 1525,
    /// nvmlVgpuTypeGetFramebufferSize
    FuncNvmlvgputypegetframebuffersize = 1526,
    /// nvmlVgpuTypeGetNumDisplayHeads
    FuncNvmlvgputypegetnumdisplayheads = 1527,
    /// nvmlVgpuTypeGetResolution
    FuncNvmlvgputypegetresolution = 1528,
    /// nvmlVgpuTypeGetLicense
    FuncNvmlvgputypegetlicense = 1529,
    /// nvmlVgpuTypeGetFrameRateLimit
    FuncNvmlvgputypegetframeratelimit = 1530,
    /// nvmlVgpuTypeGetMaxInstances
    FuncNvmlvgputypegetmaxinstances = 1531,
    /// nvmlVgpuTypeGetMaxInstancesPerVm
    FuncNvmlvgputypegetmaxinstancespervm = 1532,
    /// nvmlDeviceGetActiveVgpus
    FuncNvmldevicegetactivevgpus = 1533,
    /// nvmlVgpuInstanceGetVmID
    FuncNvmlvgpuinstancegetvmid = 1534,
    /// nvmlVgpuInstanceGetUUID
    FuncNvmlvgpuinstancegetuuid = 1535,
    /// nvmlVgpuInstanceGetVmDriverVersion
    FuncNvmlvgpuinstancegetvmdriverversion = 1536,
    /// nvmlVgpuInstanceGetFbUsage
    FuncNvmlvgpuinstancegetfbusage = 1537,
    /// nvmlVgpuInstanceGetLicenseStatus
    FuncNvmlvgpuinstancegetlicensestatus = 1538,
    /// nvmlVgpuInstanceGetType
    FuncNvmlvgpuinstancegettype = 1539,
    /// nvmlVgpuInstanceGetFrameRateLimit
    FuncNvmlvgpuinstancegetframeratelimit = 1540,
    /// nvmlVgpuInstanceGetEccMode
    FuncNvmlvgpuinstancegeteccmode = 1541,
    /// nvmlVgpuInstanceGetEncoderCapacity
    FuncNvmlvgpuinstancegetencodercapacity = 1542,
    /// nvmlVgpuInstanceSetEncoderCapacity
    FuncNvmlvgpuinstancesetencodercapacity = 1543,
    /// nvmlVgpuInstanceGetEncoderStats
    FuncNvmlvgpuinstancegetencoderstats = 1544,
    /// nvmlVgpuInstanceGetEncoderSessions
    FuncNvmlvgpuinstancegetencodersessions = 1545,
    /// nvmlVgpuInstanceGetFBCStats
    FuncNvmlvgpuinstancegetfbcstats = 1546,
    /// nvmlVgpuInstanceGetFBCSessions
    FuncNvmlvgpuinstancegetfbcsessions = 1547,
    /// nvmlVgpuInstanceGetGpuInstanceId
    FuncNvmlvgpuinstancegetgpuinstanceid = 1548,
    /// nvmlVgpuInstanceGetGpuPciId
    FuncNvmlvgpuinstancegetgpupciid = 1549,
    /// nvmlVgpuTypeGetCapabilities
    FuncNvmlvgputypegetcapabilities = 1550,
    /// nvmlVgpuInstanceGetMdevUUID
    FuncNvmlvgpuinstancegetmdevuuid = 1551,
    /// nvmlVgpuInstanceGetMetadata
    FuncNvmlvgpuinstancegetmetadata = 1552,
    /// nvmlDeviceGetVgpuMetadata
    FuncNvmldevicegetvgpumetadata = 1553,
    /// nvmlGetVgpuCompatibility
    FuncNvmlgetvgpucompatibility = 1554,
    /// nvmlDeviceGetPgpuMetadataString
    FuncNvmldevicegetpgpumetadatastring = 1555,
    /// nvmlDeviceGetVgpuSchedulerLog
    FuncNvmldevicegetvgpuschedulerlog = 1556,
    /// nvmlDeviceGetVgpuSchedulerState
    FuncNvmldevicegetvgpuschedulerstate = 1557,
    /// nvmlDeviceGetVgpuSchedulerCapabilities
    FuncNvmldevicegetvgpuschedulercapabilities = 1558,
    /// nvmlDeviceSetVgpuSchedulerState
    FuncNvmldevicesetvgpuschedulerstate = 1559,
    /// nvmlGetVgpuVersion
    FuncNvmlgetvgpuversion = 1560,
    /// nvmlSetVgpuVersion
    FuncNvmlsetvgpuversion = 1561,
    /// nvmlDeviceGetVgpuUtilization
    FuncNvmldevicegetvgpuutilization = 1562,
    /// nvmlDeviceGetVgpuInstancesUtilizationInfo
    FuncNvmldevicegetvgpuinstancesutilizationinfo = 1563,
    /// nvmlDeviceGetVgpuProcessUtilization
    FuncNvmldevicegetvgpuprocessutilization = 1564,
    /// nvmlDeviceGetVgpuProcessesUtilizationInfo
    FuncNvmldevicegetvgpuprocessesutilizationinfo = 1565,
    /// nvmlVgpuInstanceGetAccountingMode
    FuncNvmlvgpuinstancegetaccountingmode = 1566,
    /// nvmlVgpuInstanceGetAccountingPids
    FuncNvmlvgpuinstancegetaccountingpids = 1567,
    /// nvmlVgpuInstanceGetAccountingStats
    FuncNvmlvgpuinstancegetaccountingstats = 1568,
    /// nvmlVgpuInstanceClearAccountingPids
    FuncNvmlvgpuinstanceclearaccountingpids = 1569,
    /// nvmlVgpuInstanceGetLicenseInfo_v2
    FuncNvmlvgpuinstancegetlicenseinfoV2 = 1570,
    /// nvmlGetExcludedDeviceCount
    FuncNvmlgetexcludeddevicecount = 1571,
    /// nvmlGetExcludedDeviceInfoByIndex
    FuncNvmlgetexcludeddeviceinfobyindex = 1572,
    /// nvmlDeviceSetMigMode
    FuncNvmldevicesetmigmode = 1573,
    /// nvmlDeviceGetMigMode
    FuncNvmldevicegetmigmode = 1574,
    /// nvmlDeviceGetGpuInstanceProfileInfo
    FuncNvmldevicegetgpuinstanceprofileinfo = 1575,
    /// nvmlDeviceGetGpuInstanceProfileInfoV
    FuncNvmldevicegetgpuinstanceprofileinfov = 1576,
    /// nvmlDeviceGetGpuInstancePossiblePlacements_v2
    FuncNvmldevicegetgpuinstancepossibleplacementsV2 = 1577,
    /// nvmlDeviceGetGpuInstanceRemainingCapacity
    FuncNvmldevicegetgpuinstanceremainingcapacity = 1578,
    /// nvmlDeviceCreateGpuInstance
    FuncNvmldevicecreategpuinstance = 1579,
    /// nvmlDeviceCreateGpuInstanceWithPlacement
    FuncNvmldevicecreategpuinstancewithplacement = 1580,
    /// nvmlGpuInstanceDestroy
    FuncNvmlgpuinstancedestroy = 1581,
    /// nvmlDeviceGetGpuInstances
    FuncNvmldevicegetgpuinstances = 1582,
    /// nvmlDeviceGetGpuInstanceById
    FuncNvmldevicegetgpuinstancebyid = 1583,
    /// nvmlGpuInstanceGetInfo
    FuncNvmlgpuinstancegetinfo = 1584,
    /// nvmlGpuInstanceGetComputeInstanceProfileInfo
    FuncNvmlgpuinstancegetcomputeinstanceprofileinfo = 1585,
    /// nvmlGpuInstanceGetComputeInstanceProfileInfoV
    FuncNvmlgpuinstancegetcomputeinstanceprofileinfov = 1586,
    /// nvmlGpuInstanceGetComputeInstanceRemainingCapacity
    FuncNvmlgpuinstancegetcomputeinstanceremainingcapacity = 1587,
    /// nvmlGpuInstanceGetComputeInstancePossiblePlacements
    FuncNvmlgpuinstancegetcomputeinstancepossibleplacements = 1588,
    /// nvmlGpuInstanceCreateComputeInstance
    FuncNvmlgpuinstancecreatecomputeinstance = 1589,
    /// nvmlGpuInstanceCreateComputeInstanceWithPlacement
    FuncNvmlgpuinstancecreatecomputeinstancewithplacement = 1590,
    /// nvmlComputeInstanceDestroy
    FuncNvmlcomputeinstancedestroy = 1591,
    /// nvmlGpuInstanceGetComputeInstances
    FuncNvmlgpuinstancegetcomputeinstances = 1592,
    /// nvmlGpuInstanceGetComputeInstanceById
    FuncNvmlgpuinstancegetcomputeinstancebyid = 1593,
    /// nvmlComputeInstanceGetInfo_v2
    FuncNvmlcomputeinstancegetinfoV2 = 1594,
    /// nvmlDeviceIsMigDeviceHandle
    FuncNvmldeviceismigdevicehandle = 1595,
    /// nvmlDeviceGetGpuInstanceId
    FuncNvmldevicegetgpuinstanceid = 1596,
    /// nvmlDeviceGetComputeInstanceId
    FuncNvmldevicegetcomputeinstanceid = 1597,
    /// nvmlDeviceGetMaxMigDeviceCount
    FuncNvmldevicegetmaxmigdevicecount = 1598,
    /// nvmlDeviceGetMigDeviceHandleByIndex
    FuncNvmldevicegetmigdevicehandlebyindex = 1599,
    /// nvmlDeviceGetDeviceHandleFromMigDeviceHandle
    FuncNvmldevicegetdevicehandlefrommigdevicehandle = 1600,
    /// nvmlGpmMetricsGet
    FuncNvmlgpmmetricsget = 1601,
    /// nvmlGpmSampleFree
    FuncNvmlgpmsamplefree = 1602,
    /// nvmlGpmSampleAlloc
    FuncNvmlgpmsamplealloc = 1603,
    /// nvmlGpmSampleGet
    FuncNvmlgpmsampleget = 1604,
    /// nvmlGpmMigSampleGet
    FuncNvmlgpmmigsampleget = 1605,
    /// nvmlGpmQueryDeviceSupport
    FuncNvmlgpmquerydevicesupport = 1606,
    /// nvmlGpmQueryIfStreamingEnabled
    FuncNvmlgpmqueryifstreamingenabled = 1607,
    /// nvmlGpmSetStreamingEnabled
    FuncNvmlgpmsetstreamingenabled = 1608,
    /// nvmlDeviceSetNvLinkDeviceLowPowerThreshold
    FuncNvmldevicesetnvlinkdevicelowpowerthreshold = 1609,
    /// nvmlSystemSetNvlinkBwMode
    FuncNvmlsystemsetnvlinkbwmode = 1610,
    /// nvmlSystemGetNvlinkBwMode
    FuncNvmlsystemgetnvlinkbwmode = 1611,
    /// nvmlDeviceSetPowerManagementLimit_v2
    FuncNvmldevicesetpowermanagementlimitV2 = 1612,
    /// nvmlDeviceGetSramEccErrorStatus
    FuncNvmldevicegetsrameccerrorstatus = 1613,
    /// nvmlDeviceGetCapabilities
    FuncNvmldevicegetcapabilities = 1614,
    /// ncclMemAlloc
    FuncNcclmemalloc = 1615,
    /// pncclMemAlloc
    FuncPncclmemalloc = 1616,
    /// ncclMemFree
    FuncNcclmemfree = 1617,
    /// pncclMemFree
    FuncPncclmemfree = 1618,
    /// ncclGetVersion
    FuncNcclgetversion = 1619,
    /// pncclGetVersion
    FuncPncclgetversion = 1620,
    /// ncclGetUniqueId
    FuncNcclgetuniqueid = 1621,
    /// pncclGetUniqueId
    FuncPncclgetuniqueid = 1622,
    /// ncclCommInitRankConfig
    FuncNcclcomminitrankconfig = 1623,
    /// ncclCommInitRankConfigRecovery
    FuncNcclcomminitrankconfigrecovery = 1624,
    /// pncclCommInitRankConfig
    FuncPncclcomminitrankconfig = 1625,
    /// ncclCommInitRank
    FuncNcclcomminitrank = 1626,
    /// pncclCommInitRank
    FuncPncclcomminitrank = 1627,
    /// ncclCommInitAll
    FuncNcclcomminitall = 1628,
    /// pncclCommInitAll
    FuncPncclcomminitall = 1629,
    /// ncclCommFinalize
    FuncNcclcommfinalize = 1630,
    /// pncclCommFinalize
    FuncPncclcommfinalize = 1631,
    /// ncclCommDestroy
    FuncNcclcommdestroy = 1632,
    /// pncclCommDestroy
    FuncPncclcommdestroy = 1633,
    /// ncclCommAbort
    FuncNcclcommabort = 1634,
    /// pncclCommAbort
    FuncPncclcommabort = 1635,
    /// ncclCommSplit
    FuncNcclcommsplit = 1636,
    /// pncclCommSplit
    FuncPncclcommsplit = 1637,
    /// ncclCommInitRankScalable
    FuncNcclcomminitrankscalable = 1638,
    /// pncclCommInitRankScalable
    FuncPncclcomminitrankscalable = 1639,
    /// ncclGetErrorString
    FuncNcclgeterrorstring = 1640,
    /// pncclGetErrorString
    FuncPncclgeterrorstring = 1641,
    /// ncclGetLastError
    FuncNcclgetlasterror = 1642,
    /// pncclGetLastError
    FuncPncclgetlasterror = 1643,
    /// ncclCommGetAsyncError
    FuncNcclcommgetasyncerror = 1644,
    /// pncclCommGetAsyncError
    FuncPncclcommgetasyncerror = 1645,
    /// ncclCommCount
    FuncNcclcommcount = 1646,
    /// pncclCommCount
    FuncPncclcommcount = 1647,
    /// ncclCommCuDevice
    FuncNcclcommcudevice = 1648,
    /// pncclCommCuDevice
    FuncPncclcommcudevice = 1649,
    /// ncclCommUserRank
    FuncNcclcommuserrank = 1650,
    /// pncclCommUserRank
    FuncPncclcommuserrank = 1651,
    /// ncclCommRegister
    FuncNcclcommregister = 1652,
    /// pncclCommRegister
    FuncPncclcommregister = 1653,
    /// ncclCommDeregister
    FuncNcclcommderegister = 1654,
    /// pncclCommDeregister
    FuncPncclcommderegister = 1655,
    /// ncclRedOpCreatePreMulSum
    FuncNcclredopcreatepremulsum = 1656,
    /// pncclRedOpCreatePreMulSum
    FuncPncclredopcreatepremulsum = 1657,
    /// ncclRedOpDestroy
    FuncNcclredopdestroy = 1658,
    /// pncclRedOpDestroy
    FuncPncclredopdestroy = 1659,
    /// ncclReduce
    FuncNcclreduce = 1660,
    /// pncclReduce
    FuncPncclreduce = 1661,
    /// ncclBcast
    FuncNcclbcast = 1662,
    /// pncclBcast
    FuncPncclbcast = 1663,
    /// ncclBroadcast
    FuncNcclbroadcast = 1664,
    /// pncclBroadcast
    FuncPncclbroadcast = 1665,
    /// ncclAllReduce
    FuncNcclallreduce = 1666,
    /// pncclAllReduce
    FuncPncclallreduce = 1667,
    /// ncclReduceScatter
    FuncNcclreducescatter = 1668,
    /// pncclReduceScatter
    FuncPncclreducescatter = 1669,
    /// ncclAllGather
    FuncNcclallgather = 1670,
    /// pncclAllGather
    FuncPncclallgather = 1671,
    /// ncclSend
    FuncNcclsend = 1672,
    /// pncclSend
    FuncPncclsend = 1673,
    /// pncclRecv
    FuncPncclrecv = 1674,
    /// ncclRecv
    FuncNcclrecv = 1675,
    /// ncclGroupStart
    FuncNcclgroupstart = 1676,
    /// pncclGroupStart
    FuncPncclgroupstart = 1677,
    /// ncclGroupEnd
    FuncNcclgroupend = 1678,
    /// pncclGroupEnd
    FuncPncclgroupend = 1679,
    /// ncclGroupSimulateEnd
    FuncNcclgroupsimulateend = 1680,
    /// pncclGroupSimulateEnd
    FuncPncclgroupsimulateend = 1681,
    /// cuFunctionGetHash
    FuncCufunctiongethash = 1682,
}
