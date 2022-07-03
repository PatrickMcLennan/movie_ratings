import { gql } from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  DateTimeUtc: any;
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  getId: Scalars['Int'];
};


export type MutationRootGetIdArgs = {
  id: Scalars['Int'];
};

/** Public information about a user available */
export type PublicUser = {
  __typename?: 'PublicUser';
  /** Account Creation Time */
  createdAt: Scalars['DateTimeUtc'];
  /** Users email */
  email: Scalars['String'];
  /** Users first name */
  firstName: Scalars['String'];
  /** Users id */
  id: Scalars['String'];
  /** Users last name */
  lastName: Scalars['String'];
  /** Account last updated time */
  updatedAt: Scalars['DateTimeUtc'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  getAllUsers: Array<PublicUser>;
  getUserById: PublicUser;
};


export type QueryRootGetUserByIdArgs = {
  id: Scalars['String'];
};
