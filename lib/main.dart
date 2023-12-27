// ignore_for_file: avoid_print

import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_dart_callback_issue/src/rust/api/simple.dart';
import 'package:flutter_rust_bridge_dart_callback_issue/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.teal,
          brightness: Brightness.dark,
        ),
      ),
      home: Scaffold(
        appBar: AppBar(
            title: const Text('FRB: dart callback and rust print issue')),
        body: const AppBody(),
      ),
    );
  }
}

class AppBody extends StatefulWidget {
  const AppBody({super.key});

  @override
  State<AppBody> createState() => _AppBodyState();
}

class _AppBodyState extends State<AppBody> {
  final nameController = TextEditingController(text: 'test default name');

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20.0),
          child: TextField(
            controller: nameController,
            decoration: const InputDecoration(label: Text('Name')),
          ),
        ),
        const SizedBox(height: 15),
        TextButton(
          onPressed: () async {
            try {
              print('DART: before sync greet');
              final greeting = greet(name: nameController.text);
              print('DART: sync greet output: $greeting');
              showSnackbar('DART: sync greet output: $greeting');
            } catch (e) {
              print('DART: unable to sync greet: $e');
            }
          },
          child: const Text('Call Sync Greet'),
        ),
        const SizedBox(height: 5),
        TextButton(
          onPressed: () async {
            try {
              print('DART: before async greet');
              final greeting = await asyncGreet(
                name: nameController.text,
                logger: (p0) => Future.delayed(
                    const Duration(milliseconds: 100), () => print(p0)),
              );
              print('DART: async greet output: $greeting');
              if (mounted) {
                showSnackbar('DART: async greet output: $greeting');
              }
            } catch (e) {
              print('DART: unable to async greet: $e');
            }
          },
          child: const Text('Call Async Greet'),
        ),
        const SizedBox(height: 5),
        TextButton(
          onPressed: () async {
            try {
              print('DART: before async no await greet');
              final greeting = await asyncNoAwaitGreet(
                name: nameController.text,
                logger: (p0) => Future.delayed(
                    const Duration(milliseconds: 100), () => print(p0)),
              );
              print('DART: async no await greet output: $greeting');
              if (mounted) {
                showSnackbar('DART: async no await greet output: $greeting');
              }
            } catch (e) {
              print('DART: unable to async no await greet: $e');
            }
          },
          child: const Text('Call Async No Await Greet'),
        ),
        const SizedBox(height: 5),
      ],
    );
  }

  void showSnackbar(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
      ),
    );
  }
}
