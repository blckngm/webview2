

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../edge_embedded_browser/client/win/current/webview2experimental.idl:
    Oicf, W1, Zp8, env=Win64 (32b run), target_arch=AMD64 8.xx.xxxx 
    protocol : dce , ms_ext, c_ext, robust
    error checks: allocation ref bounds_check enum stub_data 
    VC __declspec() decoration level: 
         __declspec(uuid()), __declspec(selectany), __declspec(novtable)
         DECLSPEC_UUID(), MIDL_INTERFACE()
*/
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __webview2experimental_h__
#define __webview2experimental_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

/* Forward Declarations */ 

#ifndef __ICoreWebView2ExperimentalCompositionControllerInterop_FWD_DEFINED__
#define __ICoreWebView2ExperimentalCompositionControllerInterop_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalCompositionControllerInterop ICoreWebView2ExperimentalCompositionControllerInterop;

#endif 	/* __ICoreWebView2ExperimentalCompositionControllerInterop_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalCompositionController3_FWD_DEFINED__
#define __ICoreWebView2ExperimentalCompositionController3_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalCompositionController3 ICoreWebView2ExperimentalCompositionController3;

#endif 	/* __ICoreWebView2ExperimentalCompositionController3_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalCompositionController4_FWD_DEFINED__
#define __ICoreWebView2ExperimentalCompositionController4_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalCompositionController4 ICoreWebView2ExperimentalCompositionController4;

#endif 	/* __ICoreWebView2ExperimentalCompositionController4_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalController_FWD_DEFINED__
#define __ICoreWebView2ExperimentalController_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalController ICoreWebView2ExperimentalController;

#endif 	/* __ICoreWebView2ExperimentalController_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalEnvironment2_FWD_DEFINED__
#define __ICoreWebView2ExperimentalEnvironment2_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalEnvironment2 ICoreWebView2ExperimentalEnvironment2;

#endif 	/* __ICoreWebView2ExperimentalEnvironment2_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalEnvironmentInterop_FWD_DEFINED__
#define __ICoreWebView2ExperimentalEnvironmentInterop_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalEnvironmentInterop ICoreWebView2ExperimentalEnvironmentInterop;

#endif 	/* __ICoreWebView2ExperimentalEnvironmentInterop_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_FWD_DEFINED__
#define __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler;

#endif 	/* __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_FWD_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalSettings_FWD_DEFINED__
#define __ICoreWebView2ExperimentalSettings_FWD_DEFINED__
typedef interface ICoreWebView2ExperimentalSettings ICoreWebView2ExperimentalSettings;

#endif 	/* __ICoreWebView2ExperimentalSettings_FWD_DEFINED__ */


/* header files for imported files */
#include "webview2.h"

#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __WebView2Experimental_LIBRARY_DEFINED__
#define __WebView2Experimental_LIBRARY_DEFINED__

/* library WebView2Experimental */
/* [version][uuid] */ 








typedef /* [v1_enum] */ 
enum COREWEBVIEW2_BOUNDS_MODE
    {
        COREWEBVIEW2_BOUNDS_MODE_USE_RAW_PIXELS	= 0,
        COREWEBVIEW2_BOUNDS_MODE_USE_RASTERIZATION_SCALE	= ( COREWEBVIEW2_BOUNDS_MODE_USE_RAW_PIXELS + 1 ) 
    } 	COREWEBVIEW2_BOUNDS_MODE;

typedef struct COREWEBVIEW2_MATRIX_4X4
    {
    FLOAT _11;
    FLOAT _12;
    FLOAT _13;
    FLOAT _14;
    FLOAT _21;
    FLOAT _22;
    FLOAT _23;
    FLOAT _24;
    FLOAT _31;
    FLOAT _32;
    FLOAT _33;
    FLOAT _34;
    FLOAT _41;
    FLOAT _42;
    FLOAT _43;
    FLOAT _44;
    } 	COREWEBVIEW2_MATRIX_4X4;


EXTERN_C const IID LIBID_WebView2Experimental;

#ifndef __ICoreWebView2ExperimentalCompositionControllerInterop_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalCompositionControllerInterop_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalCompositionControllerInterop */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalCompositionControllerInterop = {0x4B60F2C9,0x88BB,0x42F4,{0x9C,0x4F,0x3C,0x0D,0x0E,0xD1,0x70,0x72}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B60F2C9-88BB-42F4-9C4F-3C0D0ED17072")
    ICoreWebView2ExperimentalCompositionControllerInterop : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UIAProvider( 
            /* [retval][out] */ IUnknown **provider) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootVisualTarget( 
            /* [retval][out] */ IUnknown **target) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootVisualTarget( 
            /* [in] */ IUnknown *target) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalCompositionControllerInteropVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UIAProvider )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This,
            /* [retval][out] */ IUnknown **provider);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootVisualTarget )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This,
            /* [retval][out] */ IUnknown **target);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootVisualTarget )( 
            ICoreWebView2ExperimentalCompositionControllerInterop * This,
            /* [in] */ IUnknown *target);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalCompositionControllerInteropVtbl;

    interface ICoreWebView2ExperimentalCompositionControllerInterop
    {
        CONST_VTBL struct ICoreWebView2ExperimentalCompositionControllerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalCompositionControllerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalCompositionControllerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalCompositionControllerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalCompositionControllerInterop_get_UIAProvider(This,provider)	\
    ( (This)->lpVtbl -> get_UIAProvider(This,provider) ) 

#define ICoreWebView2ExperimentalCompositionControllerInterop_get_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> get_RootVisualTarget(This,target) ) 

#define ICoreWebView2ExperimentalCompositionControllerInterop_put_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> put_RootVisualTarget(This,target) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalCompositionControllerInterop_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalCompositionController3_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalCompositionController3_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalCompositionController3 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalCompositionController3 = {0xb134916b,0xa104,0x4d2a,{0x95,0x67,0xc2,0xfd,0x53,0x71,0x43,0x50}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b134916b-a104-4d2a-9567-c2fd53714350")
    ICoreWebView2ExperimentalCompositionController3 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DragEnter( 
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DragLeave( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DragOver( 
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Drop( 
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalCompositionController3Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalCompositionController3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalCompositionController3 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalCompositionController3 * This);
        
        HRESULT ( STDMETHODCALLTYPE *DragEnter )( 
            ICoreWebView2ExperimentalCompositionController3 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        HRESULT ( STDMETHODCALLTYPE *DragLeave )( 
            ICoreWebView2ExperimentalCompositionController3 * This);
        
        HRESULT ( STDMETHODCALLTYPE *DragOver )( 
            ICoreWebView2ExperimentalCompositionController3 * This,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        HRESULT ( STDMETHODCALLTYPE *Drop )( 
            ICoreWebView2ExperimentalCompositionController3 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalCompositionController3Vtbl;

    interface ICoreWebView2ExperimentalCompositionController3
    {
        CONST_VTBL struct ICoreWebView2ExperimentalCompositionController3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalCompositionController3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalCompositionController3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalCompositionController3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalCompositionController3_DragEnter(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragEnter(This,dataObject,keyState,point,effect) ) 

#define ICoreWebView2ExperimentalCompositionController3_DragLeave(This)	\
    ( (This)->lpVtbl -> DragLeave(This) ) 

#define ICoreWebView2ExperimentalCompositionController3_DragOver(This,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragOver(This,keyState,point,effect) ) 

#define ICoreWebView2ExperimentalCompositionController3_Drop(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> Drop(This,dataObject,keyState,point,effect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalCompositionController3_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalCompositionController4_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalCompositionController4_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalCompositionController4 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalCompositionController4 = {0xe6041d7f,0x18ac,0x4654,{0xa0,0x4e,0x8b,0x3f,0x81,0x25,0x1c,0x33}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e6041d7f-18ac-4654-a04e-8b3f81251c33")
    ICoreWebView2ExperimentalCompositionController4 : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UIAProvider( 
            /* [retval][out] */ IUnknown **provider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCoreWebView2PointerInfoFromPointerId( 
            /* [in] */ UINT pointerId,
            /* [in] */ HWND parentWindow,
            /* [in] */ struct COREWEBVIEW2_MATRIX_4X4 transform,
            /* [retval][out] */ ICoreWebView2PointerInfo **pointerInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalCompositionController4Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalCompositionController4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalCompositionController4 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalCompositionController4 * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UIAProvider )( 
            ICoreWebView2ExperimentalCompositionController4 * This,
            /* [retval][out] */ IUnknown **provider);
        
        HRESULT ( STDMETHODCALLTYPE *CreateCoreWebView2PointerInfoFromPointerId )( 
            ICoreWebView2ExperimentalCompositionController4 * This,
            /* [in] */ UINT pointerId,
            /* [in] */ HWND parentWindow,
            /* [in] */ struct COREWEBVIEW2_MATRIX_4X4 transform,
            /* [retval][out] */ ICoreWebView2PointerInfo **pointerInfo);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalCompositionController4Vtbl;

    interface ICoreWebView2ExperimentalCompositionController4
    {
        CONST_VTBL struct ICoreWebView2ExperimentalCompositionController4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalCompositionController4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalCompositionController4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalCompositionController4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalCompositionController4_get_UIAProvider(This,provider)	\
    ( (This)->lpVtbl -> get_UIAProvider(This,provider) ) 

#define ICoreWebView2ExperimentalCompositionController4_CreateCoreWebView2PointerInfoFromPointerId(This,pointerId,parentWindow,transform,pointerInfo)	\
    ( (This)->lpVtbl -> CreateCoreWebView2PointerInfoFromPointerId(This,pointerId,parentWindow,transform,pointerInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalCompositionController4_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalController_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalController_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalController */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalController = {0x3413543f,0x7a0e,0x4b56,{0x98,0xc1,0x7a,0xf3,0xa3,0x74,0x16,0xca}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3413543f-7a0e-4b56-98c1-7af3a37416ca")
    ICoreWebView2ExperimentalController : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RasterizationScale( 
            /* [retval][out] */ double *scale) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RasterizationScale( 
            /* [in] */ double scale) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ShouldDetectMonitorScaleChanges( 
            /* [retval][out] */ BOOL *value) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ShouldDetectMonitorScaleChanges( 
            /* [in] */ BOOL value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE add_RasterizationScaleChanged( 
            /* [in] */ ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler *eventHandler,
            /* [out] */ EventRegistrationToken *token) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE remove_RasterizationScaleChanged( 
            /* [in] */ EventRegistrationToken token) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BoundsMode( 
            /* [retval][out] */ COREWEBVIEW2_BOUNDS_MODE *boundsMode) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BoundsMode( 
            /* [in] */ COREWEBVIEW2_BOUNDS_MODE boundsMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalControllerVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalController * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalController * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RasterizationScale )( 
            ICoreWebView2ExperimentalController * This,
            /* [retval][out] */ double *scale);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RasterizationScale )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ double scale);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ShouldDetectMonitorScaleChanges )( 
            ICoreWebView2ExperimentalController * This,
            /* [retval][out] */ BOOL *value);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ShouldDetectMonitorScaleChanges )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ BOOL value);
        
        HRESULT ( STDMETHODCALLTYPE *add_RasterizationScaleChanged )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler *eventHandler,
            /* [out] */ EventRegistrationToken *token);
        
        HRESULT ( STDMETHODCALLTYPE *remove_RasterizationScaleChanged )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ EventRegistrationToken token);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BoundsMode )( 
            ICoreWebView2ExperimentalController * This,
            /* [retval][out] */ COREWEBVIEW2_BOUNDS_MODE *boundsMode);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BoundsMode )( 
            ICoreWebView2ExperimentalController * This,
            /* [in] */ COREWEBVIEW2_BOUNDS_MODE boundsMode);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalControllerVtbl;

    interface ICoreWebView2ExperimentalController
    {
        CONST_VTBL struct ICoreWebView2ExperimentalControllerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalController_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalController_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalController_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalController_get_RasterizationScale(This,scale)	\
    ( (This)->lpVtbl -> get_RasterizationScale(This,scale) ) 

#define ICoreWebView2ExperimentalController_put_RasterizationScale(This,scale)	\
    ( (This)->lpVtbl -> put_RasterizationScale(This,scale) ) 

#define ICoreWebView2ExperimentalController_get_ShouldDetectMonitorScaleChanges(This,value)	\
    ( (This)->lpVtbl -> get_ShouldDetectMonitorScaleChanges(This,value) ) 

#define ICoreWebView2ExperimentalController_put_ShouldDetectMonitorScaleChanges(This,value)	\
    ( (This)->lpVtbl -> put_ShouldDetectMonitorScaleChanges(This,value) ) 

#define ICoreWebView2ExperimentalController_add_RasterizationScaleChanged(This,eventHandler,token)	\
    ( (This)->lpVtbl -> add_RasterizationScaleChanged(This,eventHandler,token) ) 

#define ICoreWebView2ExperimentalController_remove_RasterizationScaleChanged(This,token)	\
    ( (This)->lpVtbl -> remove_RasterizationScaleChanged(This,token) ) 

#define ICoreWebView2ExperimentalController_get_BoundsMode(This,boundsMode)	\
    ( (This)->lpVtbl -> get_BoundsMode(This,boundsMode) ) 

#define ICoreWebView2ExperimentalController_put_BoundsMode(This,boundsMode)	\
    ( (This)->lpVtbl -> put_BoundsMode(This,boundsMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalController_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalEnvironment2_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalEnvironment2_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalEnvironment2 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalEnvironment2 = {0x37b54fd4,0x1ad9,0x4c1f,{0xbd,0x14,0x9d,0xab,0xa9,0xfd,0xeb,0x26}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("37b54fd4-1ad9-4c1f-bd14-9daba9fdeb26")
    ICoreWebView2ExperimentalEnvironment2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProviderForHwnd( 
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalEnvironment2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalEnvironment2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalEnvironment2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalEnvironment2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetProviderForHwnd )( 
            ICoreWebView2ExperimentalEnvironment2 * This,
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalEnvironment2Vtbl;

    interface ICoreWebView2ExperimentalEnvironment2
    {
        CONST_VTBL struct ICoreWebView2ExperimentalEnvironment2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalEnvironment2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalEnvironment2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalEnvironment2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalEnvironment2_GetProviderForHwnd(This,hwnd,provider)	\
    ( (This)->lpVtbl -> GetProviderForHwnd(This,hwnd,provider) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalEnvironment2_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalEnvironmentInterop_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalEnvironmentInterop_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalEnvironmentInterop */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalEnvironmentInterop = {0x79455D4F,0xD28D,0x4C3F,{0xA7,0x13,0x13,0xCA,0x39,0x3B,0xD2,0xE4}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79455D4F-D28D-4C3F-A713-13CA393BD2E4")
    ICoreWebView2ExperimentalEnvironmentInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProviderForHwnd( 
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalEnvironmentInteropVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalEnvironmentInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalEnvironmentInterop * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalEnvironmentInterop * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetProviderForHwnd )( 
            ICoreWebView2ExperimentalEnvironmentInterop * This,
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalEnvironmentInteropVtbl;

    interface ICoreWebView2ExperimentalEnvironmentInterop
    {
        CONST_VTBL struct ICoreWebView2ExperimentalEnvironmentInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalEnvironmentInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalEnvironmentInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalEnvironmentInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalEnvironmentInterop_GetProviderForHwnd(This,hwnd,provider)	\
    ( (This)->lpVtbl -> GetProviderForHwnd(This,hwnd,provider) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalEnvironmentInterop_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler = {0x51560be0,0x0ad4,0x4157,{0x90,0xcf,0xe6,0xda,0xd6,0x38,0xd8,0xaa}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51560be0-0ad4-4157-90cf-e6dad638d8aa")
    ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ ICoreWebView2ExperimentalController *sender,
            /* [in] */ IUnknown *args) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalRasterizationScaleChangedEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler * This);
        
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler * This,
            /* [in] */ ICoreWebView2ExperimentalController *sender,
            /* [in] */ IUnknown *args);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalRasterizationScaleChangedEventHandlerVtbl;

    interface ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler
    {
        CONST_VTBL struct ICoreWebView2ExperimentalRasterizationScaleChangedEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_Invoke(This,sender,args)	\
    ( (This)->lpVtbl -> Invoke(This,sender,args) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalRasterizationScaleChangedEventHandler_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2ExperimentalSettings_INTERFACE_DEFINED__
#define __ICoreWebView2ExperimentalSettings_INTERFACE_DEFINED__

/* interface ICoreWebView2ExperimentalSettings */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2ExperimentalSettings = {0x684cbeef,0x47ba,0x4d4a,{0x99,0xf4,0x97,0x61,0x13,0xf9,0xf1,0x0a}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("684cbeef-47ba-4d4a-99f4-976113f9f10a")
    ICoreWebView2ExperimentalSettings : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserAgent( 
            /* [retval][out] */ LPWSTR *userAgent) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_UserAgent( 
            /* [in] */ LPCWSTR userAgent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2ExperimentalSettingsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2ExperimentalSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2ExperimentalSettings * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2ExperimentalSettings * This);
        
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAgent )( 
            ICoreWebView2ExperimentalSettings * This,
            /* [retval][out] */ LPWSTR *userAgent);
        
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_UserAgent )( 
            ICoreWebView2ExperimentalSettings * This,
            /* [in] */ LPCWSTR userAgent);
        
        END_INTERFACE
    } ICoreWebView2ExperimentalSettingsVtbl;

    interface ICoreWebView2ExperimentalSettings
    {
        CONST_VTBL struct ICoreWebView2ExperimentalSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2ExperimentalSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2ExperimentalSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2ExperimentalSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2ExperimentalSettings_get_UserAgent(This,userAgent)	\
    ( (This)->lpVtbl -> get_UserAgent(This,userAgent) ) 

#define ICoreWebView2ExperimentalSettings_put_UserAgent(This,userAgent)	\
    ( (This)->lpVtbl -> put_UserAgent(This,userAgent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2ExperimentalSettings_INTERFACE_DEFINED__ */

#endif /* __WebView2Experimental_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


