task :default => %w[build]

DIRECTORY = "cmake-build-release"

task :build do
    Dir.chdir(DIRECTORY) do
        system("cmake -DCMAKE_BUILD_TYPE=Release ..")
        system("windeployqt Nodoka.exe")
    end
end