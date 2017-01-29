

#include <QTime>
#include <QString>
#include <QStandardPaths>

namespace Core {

    void createPathIfNotExists(QString path);
    void createSettingPathIfNotExists();

    QString getSettingPath();
    QString getUniqueSettingPath(QString stringToHash);

    QString convertTimeToString(long long time);

}