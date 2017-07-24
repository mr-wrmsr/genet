{
  "variables":{
    "plugkit_sources":[
      "src/main.cpp",
      "src/plugkit.cpp",
      "src/plugkit_private.cpp",
      "src/pcap.cpp",
      "src/worker_thread.cpp",
      "src/filter_thread.cpp",
      "src/filter_thread_pool.cpp",
      "src/dissector.cpp",
      "src/dissector_thread.cpp",
      "src/dissector_thread_pool.cpp",
      "src/stream_dissector_thread.cpp",
      "src/stream_dissector_thread_pool.cpp",
      "src/listener_thread.cpp",
      "src/listener_thread_pool.cpp",
      "src/listener_status.cpp",
      "src/script_dissector.cpp",
      "src/script_stream_dissector.cpp",
      "src/stream_dissector.cpp",
      "src/listener.cpp",
      "src/plugkit_module.cpp",
      "src/variant.cpp",
      "src/slice.cpp",
      "src/frame_view.cpp",
      "src/frame_store.cpp",
      "src/filter.cpp",
      "src/fmt.cpp",
      "src/logger.cpp",
      "src/null_logger.cpp",
      "src/stream_logger.cpp",
      "src/uvloop_logger.cpp",
      "src/pcap_platform.cpp",
      "src/pcap_dummy.cpp",
      "src/wrapper/chunk_w.cpp",
      "src/wrapper/pcap_w.cpp",
      "src/wrapper/session_w.cpp",
      "src/wrapper/frame_w.cpp",
      "src/wrapper/slice_w.cpp",
      "src/wrapper/property_w.cpp",
      "src/wrapper/attribute_w.cpp",
      "src/wrapper/layer_w.cpp",
      "src/wrapper/dissector_factory_w.cpp",
      "src/wrapper/stream_dissector_factory_w.cpp",
      "src/wrapper/session_factory_w.cpp",
      "src/wrapper/listener_factory_w.cpp",
      "src/wrapper/listener_status_w.cpp",
      "src/session.cpp",
      "src/frame.cpp",
      "src/property.cpp",
      "src/attribute.cpp",
      "src/layer.cpp",
      "src/chunk.cpp",
      "vendor/json11/json11.cpp"
    ]
  },
  "targets":[
    {
      "target_name":"plugkit",
      "include_dirs":[
        "vendor/nan",
        "vendor/json11",
        "include/plugkit",
        "src"
      ],
      "sources":[
        "<@(plugkit_sources)"
      ],
      "conditions":[
        [
          "OS=='linux'",
          {
            "libraries":[
              "-Wl,-dn,-lpcap,-lcap,-lrt,-dy,-lpthread,-ldl"
            ],
            "defines":[
              "PLUGKIT_OS_LINUX"
            ]
          }
        ],
        [
          "OS=='mac'",
          {
            "libraries":[
              "-L/usr/local/lib",
              "-lpcap"
            ],
            'link_settings': {
                'libraries': [
                    '$(SDKROOT)/System/Library/Frameworks/Foundation.framework',
                    '$(SDKROOT)/System/Library/Frameworks/SystemConfiguration.framework'
                ],
            },
            "xcode_settings":{
              "MACOSX_DEPLOYMENT_TARGET":"10.9"
            },
            "defines":[
              "PLUGKIT_OS_MAC"
            ]
          }
        ],
        [
          "OS=='win'",
          {
            "include_dirs":[
              "vendor/winpcap/Include"
            ],
            "defines":[
              "PLUGKIT_OS_WIN",
              "PLUGKIT_DLL_EXPORT",
              "NOMINMAX"
            ]
          }
        ]
      ]
    }
  ]
}
