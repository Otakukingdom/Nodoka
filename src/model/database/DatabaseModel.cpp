//
// Created by mistlight on 3/9/2017.
//

#include "DatabaseModel.h"
#include <QJsonDocument>
#include <QDebug>

using namespace Database;

DatabaseModel::DatabaseModel(std::shared_ptr<Core::DatabaseInstance> dbInstance) {
    this->dbInstance = dbInstance;
}

void DatabaseModel::writeObject(QString key, QJsonObject value) {
    auto env = this->dbInstance->getDbEnv();

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);

    QJsonDocument currentDoc(value);

    QByteArray ba = key.toLocal8Bit();
    const char *keyData = ba.data();

    QString strJson(currentDoc.toJson());
    QByteArray vBa = strJson.toLocal8Bit();
    const char *valueData = vBa.data();

    qDebug() << "value set to be " << valueData;
    dbi.put(wtxn, keyData, valueData);
    wtxn.commit();
}

void DatabaseModel::writeArray(QString key, QJsonArray value) {
    auto env = this->dbInstance->getDbEnv();

    auto wtxn = lmdb::txn::begin(env);
    auto dbi = lmdb::dbi::open(wtxn, nullptr);
    dbi.put(wtxn, key, value);
    wtxn.commit();
}


void DatabaseModel::printData() {
    auto env = this->dbInstance->getDbEnv();
    auto rtxn = lmdb::txn::begin(env, nullptr, MDB_RDONLY);
    auto dbi = lmdb::dbi::open(rtxn, nullptr);

    auto cursor = lmdb::cursor::open(rtxn, dbi);

    char keyData[2048];
    char valueData[2048];
    MDB_val key = {sizeof(keyData), keyData};
    MDB_val value = {sizeof(valueData), valueData};

    while (cursor.get(&key, &value, MDB_NEXT)) {
        auto keyStr = QString::fromUtf8((char*) key.mv_data, key.mv_size);
        auto valueStr = QString::fromUtf8((char*) value.mv_data, value.mv_size);
        qDebug() << "Key is " << keyStr << " value is " << valueStr;
    }
    cursor.close();
    rtxn.abort();
}


void DatabaseModel::printValue(QString key) {
    auto env = this->dbInstance->getDbEnv();
    auto rtxn = lmdb::txn::begin(env, nullptr, MDB_RDONLY);
    auto dbi = lmdb::dbi::open(rtxn, nullptr);

    QByteArray ba = key.toLocal8Bit();
    const char *keyData = ba.data();

    char valueData[2048];
    MDB_val keyVal = {sizeof(keyData), (void*) keyData};
    MDB_val value = {sizeof(valueData), (void*) valueData};

    lmdb::dbi_get(rtxn, dbi, &keyVal, &value);
    auto valueBa = QByteArray::fromRawData((const char*) value.mv_data, value.mv_size);
    auto valueInQString = QString::fromLocal8Bit(valueBa);

    qDebug() << "value read to be ";
    qDebug() << valueInQString;
}
