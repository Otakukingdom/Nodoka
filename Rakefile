task :default => %w[build]

DIRECTORY = "cmake-build-release"
BIN_DIRECTORY = "bin"

unless File.directory?(DIRECTORY)
  FileUtils.mkdir_p(DIRECTORY)
end

task :build do
    Dir.chdir(DIRECTORY) do
        system("cmake -G \"MinGW Makefiles\" -DCMAKE_BUILD_TYPE=Release ..")
        system("mingw32-make")
        unless File.directory?(BIN_DIRECTORY)
            FileUtils.mkdir_p(BIN_DIRECTORY)
        end

        system("move Nodoka.exe #{BIN_DIRECTORY}/Nodoka.exe")

        Dir.chdir(BIN_DIRECTORY) do
            system("windeployqt Nodoka.exe --release --libdir=./../../libs/libvlc/win32 --plugindir=./../../plugins")
        end
    end
end