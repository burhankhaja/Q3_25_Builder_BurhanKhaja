import {TransactionMetadata, FailedTransactionMetadata } from "litesvm";


export const isSuccessfulTransaction = (
    result: TransactionMetadata | FailedTransactionMetadata
): result is TransactionMetadata => {
    return result.constructor.name === 'TransactionMetadata';
};

export const isFailedTransaction = (
    result: TransactionMetadata | FailedTransactionMetadata
): result is FailedTransactionMetadata => {
    return result.constructor.name === 'FailedTransactionMetadata';
};